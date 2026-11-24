export function patchTitle(source){
 const re=/(\bvoid\s+extractBackground\s*\(\s*(?:[\w.]+\.)?GuiGraphicsExtractor\s+(\w+)\s*,[^)]*\)\s*\{)/g;
 const matches=[...source.matchAll(re)];if(matches.length!==1)throw Error('TitleScreen method signature changed');
 return source.replace(re,(_,head,g)=>head+'\n      if (nova.NovaTheme.background('+g+')) return;\n');
}
export function patchLoading(source){
 const re=/(\bvoid\s+extractRenderState\s*\(\s*(?:[\w.]+\.)?GuiGraphicsExtractor\s+(\w+)\s*,[^)]*\)\s*\{)/g;
 const matches=[...source.matchAll(re)];if(matches.length!==1)throw Error('LoadingOverlay method signature changed');
 // Keep the original implementation in a renamed method. A wrapper preserves the overlay.
 const m=matches[0],g=m[2];const declaration=m[1].replace(/\bextractRenderState\b/,'nova$originalRender');
 const params=m[1].slice(m[1].indexOf('(')+1,m[1].lastIndexOf(')')).split(',').map(p=>p.trim().split(/\s+/).at(-1));
 if(params.length!==4)throw Error('Unexpected loading parameters');
 // Avoid leaving @Override on the renamed method: the original signature is retained as wrapper.
 return source.replace(re,head=>head+'\n      nova$originalRender('+params.join(', ')+');\n      nova.NovaTheme.loading('+g+');\n   }\n   private '+declaration);
}
