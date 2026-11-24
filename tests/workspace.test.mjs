import test from 'node:test';import assert from 'node:assert/strict';import fs from 'node:fs/promises';
test('local Minecraft workspace is ignored',async()=>assert.match(await fs.readFile('.gitignore','utf8'),/\.nova-work\//));
test('native bootstrap does not depend on Fabric',async()=>assert.doesNotMatch(await fs.readFile('src/main/java/nova/nativeclient/Bootstrap.java','utf8'),/net\.fabricmc/));
