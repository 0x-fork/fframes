/* TypeScript file generated from EditorContext.res by genType. */
/* eslint-disable import/first */


import * as React from 'react';

// @ts-ignore: Implicit any on import
import * as EditorContextBS__Es6Import from './EditorContext.bs';
const EditorContextBS: any = EditorContextBS__Es6Import;

import type {t as WasmController_t} from './WasmController.gen';

import type {videoMeta as WasmController_videoMeta} from './WasmController.gen';

// tslint:disable-next-line:interface-over-type-literal
export type playState = 
    "Playing"
  | "Paused"
  | "WaitingForAction"
  | "CantPlay";

// tslint:disable-next-line:interface-over-type-literal
export type editorState = {
  readonly frame: number; 
  readonly playState: playState; 
  readonly svg?: string
};

// tslint:disable-next-line:interface-over-type-literal
export type editorContext = {
  readonly wasmController: WasmController_t; 
  readonly videoMeta: WasmController_videoMeta; 
  readonly editorState: editorState
};

// tslint:disable-next-line:interface-over-type-literal
export type Props = {
  readonly children: React.ReactNode; 
  readonly videoMeta: WasmController_videoMeta; 
  readonly wasmController: WasmController_t
};

export const EditorContext_make: React.ComponentType<{
  readonly children: React.ReactNode; 
  readonly videoMeta: WasmController_videoMeta; 
  readonly wasmController: WasmController_t
}> = EditorContextBS.EditorContext.make;

export const EditorContext: { make: React.ComponentType<{
  readonly children: React.ReactNode; 
  readonly videoMeta: WasmController_videoMeta; 
  readonly wasmController: WasmController_t
}> } = EditorContextBS.EditorContext
