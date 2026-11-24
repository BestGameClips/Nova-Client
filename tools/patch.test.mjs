import test from 'node:test';import assert from 'node:assert/strict';import {patchTitle,patchLoading} from './patch.mjs';
test('title patch',()=>assert.match(patchTitle('public void extractBackground(GuiGraphicsExtractor g, int x, int y, float f) { old(); }'),/nova\.NovaTheme\.background\(g\)/));
test('loading patch',()=>assert.match(patchLoading('public void extractRenderState(GuiGraphicsExtractor g, int x, int y, float f) { old(); }'),/private void nova\$originalRender/));
test('fails closed',()=>assert.throws(()=>patchTitle('unmatched')));
