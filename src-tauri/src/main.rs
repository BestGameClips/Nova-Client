#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{path::{Path,PathBuf},process::Command,io::{Read,Write}};
use sha2::{Sha256,Digest};
use tokio::io::AsyncWriteExt;
const MC:&str="26.3";

type ResultS<T>=Result<T,String>;
fn err(e:impl std::fmt::Display)->String{e.to_string()}
async fn download(c:&reqwest::Client,url:&str,dest:&Path)->ResultS<String>{
 let mut r=c.get(url).send().await.map_err(err)?.error_for_status().map_err(err)?;
 let mut f=tokio::fs::File::create(dest).await.map_err(err)?;
 let mut hash=Sha256::new();let mut size=0u64;
 while let Some(chunk)=r.chunk().await.map_err(err)?{size+=chunk.len() as u64;if size>500_000_000{return Err("Téléchargement trop volumineux".into())}hash.update(&chunk);f.write_all(&chunk).await.map_err(err)?;}
 f.flush().await.map_err(err)?;Ok(format!("{:x}",hash.finalize()))
}
fn unzip_java(archive:&Path,root:&Path)->ResultS<PathBuf>{
 let f=std::fs::File::open(archive).map_err(err)?;let mut z=zip::ZipArchive::new(f).map_err(err)?;
 let mut total=0u64;
 for i in 0..z.len(){let mut entry=z.by_index(i).map_err(err)?;let path=entry.enclosed_name().ok_or("Archive invalide")?;let target=root.join(path);total=total.checked_add(entry.size()).ok_or("Archive trop grande")?;if total>1_500_000_000{return Err("Archive trop grande".into())}if entry.is_dir(){std::fs::create_dir_all(&target).map_err(err)?;}else{if let Some(p)=target.parent(){std::fs::create_dir_all(p).map_err(err)?;}let mut out=std::fs::File::create(target).map_err(err)?;std::io::copy(&mut entry,&mut out).map_err(err)?;}}
 for e in std::fs::read_dir(root).map_err(err)?{let p=e.map_err(err)?.path().join("bin").join("java.exe");if p.is_file(){return Ok(p)}}Err("Java introuvable après extraction".into())
}

async fn json(c:&reqwest::Client,url:&str)->ResultS<serde_json::Value>{c.get(url).send().await.map_err(err)?.error_for_status().map_err(err)?.json().await.map_err(err)}
fn s<'a>(v:&'a serde_json::Value,k:&str)->ResultS<&'a str>{v[k].as_str().ok_or_else(||format!("Missing {k}"))}
async fn verified(c:&reqwest::Client,url:&str,dest:&Path,expected:&str)->ResultS<()>{
 use sha1::Sha1;
 if let Ok(b)=tokio::fs::read(dest).await{if format!("{:x}",Sha1::digest(&b))==expected{return Ok(())}}
 if !url.starts_with("https://"){return Err("HTTPS required".into())}
 if let Some(p)=dest.parent(){tokio::fs::create_dir_all(p).await.map_err(err)?;}
 let tmp=dest.with_extension("nova-part");
 download(c,url,&tmp).await?;
 let b=tokio::fs::read(&tmp).await.map_err(err)?;
 if format!("{:x}",Sha1::digest(&b))!=expected{let _=tokio::fs::remove_file(&tmp).await;return Err(format!("Checksum incorrect: {}",dest.display()))}
 tokio::fs::rename(tmp,dest).await.map_err(err)?;Ok(())
}
fn safe_join(root:&Path,relative:&str)->ResultS<PathBuf>{
 let p=Path::new(relative);if p.components().any(|c|!matches!(c,std::path::Component::Normal(_))){return Err("Chemin non autorisé".into())}Ok(root.join(p))
}
fn allowed(v:&serde_json::Value)->bool{
 let Some(rules)=v["rules"].as_array()else{return true};
 let mut allow=false;
 for r in rules{let os=&r["os"];if !r["features"].is_null(){continue}if let Some(n)=os["name"].as_str(){if n!="windows"{continue}}if let Some(a)=os["arch"].as_str(){if a!="x86_64"&&a!="amd64"{continue}}if !os["version"].is_null(){continue}allow=r["action"]=="allow";}
 allow
}
async fn artifact(c:&reqwest::Client,root:&Path,a:&serde_json::Value)->ResultS<PathBuf>{
 let dest=safe_join(root,s(a,"path")?)?;verified(c,s(a,"url")?,&dest,s(a,"sha1")?).await?;Ok(dest)
}
#[tauri::command]
async fn install_native(folder:String)->ResultS<String>{
 if !cfg!(all(target_os="windows",target_arch="x86_64")){return Err("Windows x64 uniquement".into())}
 let game=PathBuf::from(folder);if !game.is_absolute()||!game.is_dir(){return Err("Dossier invalide".into())}
 let c=reqwest::Client::builder().timeout(std::time::Duration::from_secs(600)).build().map_err(err)?;
 let cache=game.join("runtime");tokio::fs::create_dir_all(&cache).await.map_err(err)?;
 let java_dir=cache.join("java");let mut existing=None;
 if let Ok(entries)=std::fs::read_dir(&java_dir){for e in entries.flatten(){let p=e.path().join("bin/java.exe");if p.is_file(){let valid=Command::new(&p).arg("-version").output().map(|o|o.status.success()&&String::from_utf8_lossy(&o.stderr).contains("version \"25")).unwrap_or(false);if valid{existing=Some(p);break}}}}
 let java=if let Some(p)=existing{p}else{
 let meta:serde_json::Value=c.get("https://api.adoptium.net/v3/assets/latest/25/hotspot?architecture=x64&image_type=jdk&os=windows&vendor=eclipse").send().await.map_err(err)?.error_for_status().map_err(err)?.json().await.map_err(err)?;
 let p=&meta[0]["binary"]["package"];let url=p["link"].as_str().ok_or("URL Java absente")?;let expected=p["checksum"].as_str().ok_or("Checksum Java absent")?;
 let archive=cache.join("jdk.zip");let got=download(&c,url,&archive).await?;if !got.eq_ignore_ascii_case(expected){return Err("Checksum Java incorrect".into())}
 tokio::fs::create_dir_all(&java_dir).await.map_err(err)?;
 let j=tokio::task::spawn_blocking(move||unzip_java(&archive,&java_dir)).await.map_err(err)??;j};

 let manifest=json(&c,"https://piston-meta.mojang.com/mc/game/version_manifest_v2.json").await?;
 let entry=manifest["versions"].as_array().and_then(|a|a.iter().find(|v|v["id"]==MC)).ok_or("Minecraft 26.3 indisponible dans le manifeste officiel")?;
 let versions=game.join("versions").join(MC);
 let metadata=versions.join(format!("{MC}.json"));
 verified(&c,s(entry,"url")?,&metadata,s(entry,"sha1")?).await?;
 let v:serde_json::Value=serde_json::from_slice(&tokio::fs::read(&metadata).await.map_err(err)?).map_err(err)?;
 if v["javaVersion"]["majorVersion"]!=25{return Err("Version Java inattendue".into())}
 let client=&v["downloads"]["client"];
 verified(&c,s(client,"url")?,&versions.join(format!("{MC}.jar")),s(client,"sha1")?).await?;
 let mut cp=Vec::new();
 let libs=v["libraries"].as_array().ok_or("Bibliothèques absentes")?;
 for lib in libs{if !allowed(lib){continue}if let Some(a)=lib["downloads"].get("artifact"){cp.push(artifact(&c,&game.join("libraries"),a).await?);}
 if let Some(native)=lib["natives"]["windows"].as_str(){let key=native.replace("${arch}","64");if let Some(a)=lib["downloads"]["classifiers"].get(&key){let jar=artifact(&c,&game.join("libraries"),a).await?;let dest=game.join("natives");tokio::fs::create_dir_all(&dest).await.map_err(err)?;tokio::task::spawn_blocking(move||->ResultS<()>{let mut z=zip::ZipArchive::new(std::fs::File::open(jar).map_err(err)?).map_err(err)?;for i in 0..z.len(){let mut e=z.by_index(i).map_err(err)?;let Some(p)=e.enclosed_name()else{continue};if e.is_dir()||p.starts_with("META-INF"){continue}let out=dest.join(p);if let Some(p)=out.parent(){std::fs::create_dir_all(p).map_err(err)?;}std::io::copy(&mut e,&mut std::fs::File::create(out).map_err(err)?).map_err(err)?;}Ok(())}).await.map_err(err)??;}}
 }
 let index=&v["assetIndex"];let id=s(index,"id")?;
 let indexpath=safe_join(&game.join("assets/indexes"),&format!("{id}.json"))?;
 verified(&c,s(index,"url")?,&indexpath,s(index,"sha1")?).await?;
 let idx:serde_json::Value=serde_json::from_slice(&tokio::fs::read(indexpath).await.map_err(err)?).map_err(err)?;
 for obj in idx["objects"].as_object().ok_or("Index assets invalide")?.values(){let hash=s(obj,"hash")?;if hash.len()!=40||!hash.bytes().all(|b|b.is_ascii_hexdigit()){return Err("Hash invalide".into())}let rel=format!("{}/{}",&hash[..2],hash);verified(&c,&format!("https://resources.download.minecraft.net/{rel}"),&game.join("assets/objects").join(rel),hash).await?;}
 let exe=std::env::current_exe().map_err(err)?;
 let adjacent=exe.parent().ok_or("Dossier EXE absent")?.join("nova-client.jar");
 let agent=game.join("nova-client.jar");
 if adjacent.is_file()&&adjacent!=agent{tokio::fs::copy(adjacent,&agent).await.map_err(err)?;}
 if !agent.is_file(){return Err(format!("Place nova-client.jar dans {} ou à côté de NovaLauncher.exe, puis relance.",game.display()))}
 Ok(format!("Minecraft et dépendances installés. Java : {}. Le lancement avec authentification Microsoft reste à intégrer.",java.display()))
}
fn main(){tauri::Builder::default().plugin(tauri_plugin_dialog::init()).invoke_handler(tauri::generate_handler![install_native]).run(tauri::generate_context!()).expect("Nova Launcher");}
