/* TypeScript file generated from Player.res by genType. */
/* eslint-disable import/first */


// tslint:disable-next-line:interface-over-type-literal
export type playState = 
    "Playing"
  | "Paused"
  | "WaitingForAction"
  | "CantPlay";

// tslint:disable-next-line:interface-over-type-literal
export type state = {
  readonly frame: number; 
  readonly startPlayingFrame: number; 
  readonly playState: playState; 
  readonly svg?: string
};

// tslint:disable-next-line:interface-over-type-literal
export type action = 
    "AllowPlay"
  | "Play"
  | "Pause"
  | { tag: "NewFrame"; value: number };
