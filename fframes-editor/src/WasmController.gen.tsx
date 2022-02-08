/* TypeScript file generated from WasmController.res by genType. */
/* eslint-disable import/first */


import type {Js_BigInt_t as ReScriptJs_Js_BigInt_t} from './shims/Js.shim';

import type {Js_Dict_t as ReScriptJs_Js_Dict_t} from './shims/Js.shim';

import type {Js_Float32Array_t as ReScriptJs_Js_Float32Array_t} from './shims/Js.shim';

import type {Js_Promise_t as ReScriptJs_Js_Promise_t} from './shims/Js.shim';

// tslint:disable-next-line:interface-over-type-literal
export type videoMeta = {
  readonly name: string; 
  readonly width: number; 
  readonly height: number; 
  readonly fps: number; 
  readonly durationInFrames: number; 
  readonly audioMap?: ReScriptJs_Js_Dict_t<[number, number]>
};
export type VideoMeta = videoMeta;

// tslint:disable-next-line:interface-over-type-literal
export type t = {
  readonly add_audio_source: (_1:string, _2:ReScriptJs_Js_Float32Array_t) => void; 
  readonly add_image_source: (_1:string, _2:string) => void; 
  readonly add_subtitles_source: (_1:string, _2:string) => number; 
  readonly default: () => ReScriptJs_Js_Promise_t<void>; 
  readonly prepare: () => ReScriptJs_Js_Promise_t<videoMeta>; 
  readonly render_frame: (_1:ReScriptJs_Js_BigInt_t) => string
};
export type WasmController = t;
