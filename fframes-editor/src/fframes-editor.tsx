import "../tw.css";
import * as React from "react";
import * as ReactDOM from "react-dom";
import { Editor } from "./Editor.gen";
import { EditorContext } from "./EditorContext.gen";
import type { WasmController } from "./WasmController.gen";

export function renderEditor(wasm: WasmController) {
  wasm.default().then(() =>
    wasm.prepare().then((videoMeta) => {
      ReactDOM
        // @ts-expect-error REACT 18 BINDINGS are missing aaaaa
        .createRoot(document.getElementById("root"))
        .render(
          <EditorContext.make videoMeta={videoMeta} wasmController={wasm}>
            <Editor />
          </EditorContext.make>
        );
    })
  );
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
