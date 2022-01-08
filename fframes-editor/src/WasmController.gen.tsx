/* TypeScript file generated from WasmController.res by genType. */
/* eslint-disable import/first */


import type {Js_Float32Array_t as ReScriptJs_Js_Float32Array_t} from './shims/Js.shim';

import type {Js_Promise_t as ReScriptJs_Js_Promise_t} from './shims/Js.shim';

// tslint:disable-next-line:interface-over-type-literal
export type videoMeta = {
  readonly name: string; 
  readonly width: number; 
  readonly height: number; 
  readonly duration: number
};
export type VideoMeta = videoMeta;

// tslint:disable-next-line:interface-over-type-literal
export type t = {
  readonly add_audio_source: (_1:string, _2:ReScriptJs_Js_Float32Array_t) => void; 
  readonly add_subtitles_source: (_1:string, _2:string) => void; 
  readonly default: () => ReScriptJs_Js_Promise_t<void>; 
  readonly prepare: () => ReScriptJs_Js_Promise_t<videoMeta>
};
export type WasmController = t;
