import fs from 'node:fs/promises';import path from 'node:path';import crypto from 'node:crypto';import {spawnSync} from 'node:child_process';import {fileURLToPath} from 'node:url';import {patchTitle,patchLoading} from './patch.mjs';
const [gameArg,javaArg]=process.argv.slice(2);if(!gameArg||!javaArg)throw Error('Usage: recompile.mjs <game> <java.exe>');
const game=path.resolve(gameArg),java=path.resolve(javaArg),jdk=path.dirname(java),tools=path.dirname(fileURLToPath(import.meta.url));
const work=path.join(game,'.nova-work'),sources=path.join(work,'sources'),classes=path.join(work,'classes'),client=path.join(game,'versions','26.3','26.3.jar');
await fs.mkdir(work,{recursive:true});await fs.rm(sources,{recursive:true,force:true});await fs.rm(classes,{recursive:true,force:true});await fs.mkdir(sources,{recursive:true});await fs.mkdir(classes,{recursive:true});
async function fetchBytes(url){const r=await fetch(url,{signal:AbortSignal.timeout(180000)});if(!r.ok)throw Error('HTTP '+r.status+' '+url);return Buffer.from(await r.arrayBuffer());}
const url='https://repo.maven.apache.org/maven2/org/vineflower/vineflower/1.12.0/vineflower-1.12.0.jar',vf=path.join(work,'vineflower.jar');
const bytes=await fetchBytes(url),sha=(await fetchBytes(url+'.sha1')).toString().trim().split(/\s+/)[0];if(crypto.createHash('sha1').update(bytes).digest('hex')!==sha.toLowerCase())throw Error('Vineflower checksum mismatch');await fs.writeFile(vf,bytes);
function run(exe,args){const r=spawnSync(exe,args,{stdio:'inherit',windowsHide:true});if(r.error)throw r.error;if(r.status!==0)throw Error(path.basename(exe)+' exited '+r.status);}
async function jars(dir){let out=[];for(const e of await fs.readdir(dir,{withFileTypes:true})){const p=path.join(dir,e.name);if(e.isDirectory())out.push(...await jars(p));else if(e.isFile()&&p.endsWith('.jar'))out.push(p);}return out;}
const metadata=JSON.parse(await fs.readFile(path.join(game,'versions','26.3','26.3.json'),'utf8'));
const libs=[];for(const l of metadata.libraries||[]){const a=l.downloads?.artifact;if(a?.path){const p=path.join(game,'libraries',a.path);try{await fs.access(p);libs.push(p)}catch{}}}
const targets=['net/minecraft/client/gui/screens/TitleScreen','net/minecraft/client/gui/screens/LoadingOverlay'];
run(java,['-Xmx2G','-jar',vf,'--folder','--skip-extra-files=1','--thread-count=2',...targets.map(t=>'--only='+t),client,sources]);
const sourcePaths=targets.map(t=>path.join(sources,t+'.java'));
for(let i=0;i<sourcePaths.length;i++){const p=sourcePaths[i];await fs.writeFile(p,[patchTitle,patchLoading][i](await fs.readFile(p,'utf8')));}
const cp=[client,...libs].join(path.delimiter);const quote=s=>'"'+s.replaceAll('\\','/').replaceAll('"','\\"')+'"';
const args=['--release','25','-encoding','UTF-8','-classpath',cp,'-d',classes,...sourcePaths,path.join(tools,'NovaTheme.java')];
const argfile=path.join(work,'javac.args');await fs.writeFile(argfile,args.map(quote).join('\n'));
run(path.join(jdk,'javac.exe'),['@'+argfile]);
const temp=path.join(work,'nova-rebuilt.jar');await fs.copyFile(client,temp);run(path.join(jdk,'jar.exe'),['--update','--file',temp,'-C',classes,'.']);
await fs.copyFile(temp,path.join(game,'nova-client.jar'));console.log('Local Nova jar generated');
