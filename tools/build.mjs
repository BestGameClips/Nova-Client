import fs from 'node:fs/promises';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
const ext=process.platform==='win32'?'.exe':'';
const bin=n=>process.env.JAVA_HOME?path.join(process.env.JAVA_HOME,'bin',n+ext):n;
const run=(cmd,args)=>{const r=spawnSync(cmd,args,{stdio:'inherit'});if(r.status!==0)throw Error(cmd+' failed');};
await fs.mkdir('build/classes',{recursive:true});
run(bin('javac'),['--release','25','-d','build/classes','src/main/java/nova/nativeclient/Bootstrap.java']);
run(bin('jar'),['--create','--file','build/nova-client.jar','--manifest','src/main/resources/MANIFEST.MF','-C','build/classes','.']);
