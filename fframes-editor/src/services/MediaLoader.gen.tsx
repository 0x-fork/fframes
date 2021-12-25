/* TypeScript file generated from MediaLoader.res by genType. */
/* eslint-disable import/first */


import type {Js_ArrayBuffer_t as ReScriptJs_Js_ArrayBuffer_t} from './shims/Js.shim';

import type {Js_Promise_t as ReScriptJs_Js_Promise_t} from './shims/Js.shim';

import type {t as WasmController_t} from '../../src/WasmController.gen';

// tslint:disable-next-line:interface-over-type-literal
export type audioInfo = {
  readonly duration: number; 
  readonly sampleRate: number; 
  readonly arrayBuffer: ReScriptJs_Js_ArrayBuffer_t
};

// tslint:disable-next-line:interface-over-type-literal
export type imageInfo = { readonly width: number; readonly height: number };

// tslint:disable-next-line:interface-over-type-literal
export type processedMedia = 
    { tag: "Font"; value: string }
  | { tag: "Subtitles"; value: string }
  | { tag: "Image"; value: [string, imageInfo] }
  | { tag: "Audio"; value: [string, audioInfo] };

// tslint:disable-next-line:interface-over-type-literal
export type mediaResolveFn = (_1:{ readonly name: string; readonly url: string }, _2:WasmController_t) => ReScriptJs_Js_Promise_t<processedMedia>;
export type MediaResolver = mediaResolveFn;
