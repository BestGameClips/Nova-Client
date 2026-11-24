import fs from 'node:fs/promises';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const work=path.join(root,'.nova-work');
const version='26.3';
await fs.mkdir(work,{recursive:true});
async function fetchBytes(url){
 const u=new URL(url);
 if(u.protocol!=='https:')throw Error('HTTPS required');
 const r=await fetch(u,{signal:AbortSignal.timeout(180000)});
 if(!r.ok)throw Error('HTTP '+r.status+' '+u);
 return Buffer.from(await r.arrayBuffer());
}
async function verified(url,dest,sha1,size){
 async function valid(b){return (size==null||b.length===size)&&crypto.createHash('sha1').update(b).digest('hex')===sha1;}
 let bytes;try{bytes=await fs.readFile(dest)}catch{}
 if(!bytes||!await valid(bytes)){bytes=await fetchBytes(url);if(!await valid(bytes))throw Error('Integrity verification failed: '+dest);await fs.writeFile(dest,bytes);}
 return bytes;
}
const manifest=JSON.parse(await fetchBytes('https://piston-meta.mojang.com/mc/game/version_manifest_v2.json'));
const entry=manifest.versions.find(v=>v.id===version);
if(!entry)throw Error('Minecraft '+version+' unavailable in official manifest');
const metadata=JSON.parse(await verified(entry.url,path.join(work,'version.json'),entry.sha1));
if(metadata.javaVersion?.majorVersion!==25)throw Error('Unexpected Java version');
const d=metadata.downloads.client;
const jar=path.join(work,'minecraft-client.jar');
await verified(d.url,jar,d.sha1,d.size);
console.log('Verified Minecraft '+version+' client downloaded locally.');
if(process.argv.includes('--decompile')){
 const cfr=path.join(work,'cfr-0.152.jar');
 const base='https://repo.maven.apache.org/maven2/org/benf/cfr/0.152/cfr-0.152.jar';
 const hash=(await fetchBytes(base+'.sha1')).toString().trim();
 if(!/^[a-f0-9]{40}$/i.test(hash))throw Error('Invalid CFR checksum');
 await verified(base,cfr,hash.toLowerCase());
 const java=process.env.JAVA_HOME?path.join(process.env.JAVA_HOME,'bin',process.platform==='win32'?'java.exe':'java'):'java';
 const check=spawnSync(java,['-version'],{encoding:'utf8'});
 if(check.status!==0||!/(?:version "|openjdk )25(?:[.+" ]|$)/.test(check.stderr+check.stdout))throw Error('Java 25 required');
 const result=spawnSync(java,['-jar',cfr,jar,'--outputdir',path.join(work,'sources')],{stdio:'inherit'});
 if(result.status!==0)throw Error('CFR failed; use a newer decompiler if necessary');
 console.log('Local sources: '+path.join(work,'sources'));
}
