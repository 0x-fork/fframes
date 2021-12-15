import "../tw.css";
import * as React from "react";
import * as ReactDOM from "react-dom";
import { Editor } from "./Editor.gen";

export interface VideoMetadata {
  readonly name: string;
}

interface Wasm {
  default(): Promise<unknown>;
  prepare(): Promise<VideoMetadata>;
}

export function renderEditor(wasm: Wasm) {
  wasm.default().then(() => wasm.prepare().then(console.log));
  ReactDOM
    // @ts-expect-error REACT 18 BINDINGS are missing aaaaa
    .createRoot(document.getElementById("root"))
    .render(<Editor />);
}

export async function load_audio_wasm_callback(name: string) {
  const response = await fetch(`/media/${name}`);

  if (!response.ok) {
    throw new Error(
      `Can not load ${name}. Error: ${response.status}(${response.statusText})`
    );
  }

  const context = new AudioContext();

  try {
    const audioBuffer = await context.decodeAudioData(
      await response.arrayBuffer()
    );

    return audioBuffer.duration;
  } catch (e) {
    throw new Error(
      `Can process audio file "${name}". Maybe it's not an audio? ${e}`
    );
  }
}
