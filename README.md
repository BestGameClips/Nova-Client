# Nova Native — 0.2 development foundation
This branch replaces the Fabric prototype with a local Minecraft development workspace and an original Java agent bootstrap. It is NOT a playable client yet.

Requirements: Java 25, Node.js 22+, legitimate Minecraft Java account for playing.
Run prepare-native.bat (Windows) or npm run decompile to fetch the official 26.3 client, validate Mojang SHA-1 and size, and attempt local CFR decompilation. CFR 0.152 predates Java 25, so decompilation may fail or produce incomplete code. Generated game files are ignored and must not be uploaded.
npm test checks basic workspace invariants. npm run build compiles the original bootstrap agent only. CI artifact is not the Minecraft game and cannot be launched standalone.
Next: inspect local mappings/classes, implement verified bytecode transformations for custom screens and Discord presence, acquire libraries/assets, implement authorized Microsoft sign-in and entitlement checks in launcher. No fake accounts or tokens. Offline play requires prior legitimate authentication and installed resources.
No Fabric dependency in this branch. Previous Fabric client and Tauri launcher remain in their existing branches, not connected to this bootstrap yet.
Minecraft EULA: https://www.minecraft.net/en-us/eula
