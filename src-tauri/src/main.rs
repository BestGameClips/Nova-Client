#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{path::{Path,PathBuf},process::Command,io::{Read,Write}};
use sha2::{Sha256,Digest};
use tokio::io::AsyncWriteExt;
const MC:&str="26.3";
const LOADER:&str="0.19.5";
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
#[tauri::command]
async fn install_nova(folder:String,jar:String)->ResultS<String>{
 if !cfg!(target_os="windows"){return Err("Windows uniquement".into())}
 if !cfg!(target_arch="x86_64"){return Err("Cette version nécessite Windows x64".into())}
 let game=PathBuf::from(folder);let source=PathBuf::from(jar);
 if !game.is_absolute()||!game.is_dir()||!source.is_file(){return Err("Dossier ou JAR invalide".into())}
 let appdata=PathBuf::from(std::env::var_os("APPDATA").ok_or("APPDATA absent")?);
 let mc=appdata.join(".minecraft");
 if !mc.join("launcher_profiles.json").is_file()&&!mc.join("launcher_profiles_microsoft_store.json").is_file(){return Err("Installe et ouvre une première fois le launcher officiel Minecraft, puis ferme-le.".into())}
 let c=reqwest::Client::builder().timeout(std::time::Duration::from_secs(600)).build().map_err(err)?;
 let cache=game.join(".nova-runtime");tokio::fs::create_dir_all(&cache).await.map_err(err)?;
 let java_dir=cache.join("java");let mut existing=None;
 if let Ok(entries)=std::fs::read_dir(&java_dir){for e in entries.flatten(){let p=e.path().join("bin/java.exe");if p.is_file(){let valid=Command::new(&p).arg("-version").output().map(|o|o.status.success()&&String::from_utf8_lossy(&o.stderr).contains("version \"25")).unwrap_or(false);if valid{existing=Some(p);break}}}}
 let java=if let Some(p)=existing{p}else{
 let meta:serde_json::Value=c.get("https://api.adoptium.net/v3/assets/latest/25/hotspot?architecture=x64&image_type=jdk&os=windows&vendor=eclipse").send().await.map_err(err)?.error_for_status().map_err(err)?.json().await.map_err(err)?;
 let p=&meta[0]["binary"]["package"];let url=p["link"].as_str().ok_or("URL Java absente")?;let expected=p["checksum"].as_str().ok_or("Checksum Java absent")?;
 let archive=cache.join("jdk.zip");let got=download(&c,url,&archive).await?;if !got.eq_ignore_ascii_case(expected){return Err("Checksum Java incorrect".into())}
 tokio::fs::create_dir_all(&java_dir).await.map_err(err)?;
 let j=tokio::task::spawn_blocking(move||unzip_java(&archive,&java_dir)).await.map_err(err)??;j};
 let versions:serde_json::Value=c.get(format!("https://meta.fabricmc.net/v2/versions/loader/{MC}")).send().await.map_err(err)?.error_for_status().map_err(err)?.json().await.map_err(err)?;
 if !versions.as_array().map(|a|a.iter().any(|v|v["loader"]["version"].as_str()==Some(LOADER))).unwrap_or(false){return Err("Fabric Loader non disponible pour cette version".into())}
 let installers:serde_json::Value=c.get("https://meta.fabricmc.net/v2/versions/installer").send().await.map_err(err)?.error_for_status().map_err(err)?.json().await.map_err(err)?;
 let item=installers.as_array().and_then(|a|a.iter().find(|v|v["stable"].as_bool()==Some(true))).ok_or("Installateur Fabric absent")?;
 let url=item["url"].as_str().ok_or("URL Fabric absente")?;
 if !url.starts_with("https://maven.fabricmc.net/"){return Err("Source Fabric inattendue".into())}
 let installer=cache.join("fabric-installer.jar");download(&c,url,&installer).await?;
 let j=java.clone();let m=mc.clone();let output=tokio::task::spawn_blocking(move||Command::new(j).arg("-jar").arg(installer).arg("client").arg("-dir").arg(m).args(["-mcversion",MC,"-loader",LOADER]).output()).await.map_err(err)?.map_err(err)?;
 if !output.status.success(){return Err(format!("Fabric : {}",String::from_utf8_lossy(&output.stderr)))}
 let mods=game.join("mods");tokio::fs::create_dir_all(&mods).await.map_err(err)?;let dest=mods.join("nova-client.jar");
 if source!=dest{tokio::fs::copy(&source,&dest).await.map_err(err)?;}
 Ok(format!("Installation terminée. Dans Minecraft Launcher, sélectionne Fabric {MC}, définis le dossier de jeu : {}\nExécutable Java : {}\nFerme et relance le launcher si nécessaire.",game.display(),java.display()))
}
#[tauri::command]
fn open_launcher()->ResultS<()>{
 #[cfg(target_os="windows")] {Command::new("explorer.exe").arg("minecraft://").spawn().map_err(err)?;Ok(())}
 #[cfg(not(target_os="windows"))] {Err("Windows uniquement".into())}
}
fn main(){tauri::Builder::default().plugin(tauri_plugin_dialog::init()).invoke_handler(tauri::generate_handler![install_nova,open_launcher]).run(tauri::generate_context!()).expect("Nova Launcher");}
