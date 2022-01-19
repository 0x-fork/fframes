import "../tw.css";
import * as React from "react";
import * as ReactDOM from "react-dom";
import { Editor } from "./ui/Editor.gen";
import { EditorContext, Props } from "./EditorContext.gen";
import type { WasmController } from "./WasmController.gen";
import { processImports } from "./services/mediaLoader.gen";

export function renderEditor(
  imports: Parameters<typeof processImports>[0]["imports"],
  wasmController: WasmController
) {
  wasmController.default().then(() => {
    Promise.all([
      processImports({ imports, wasmController }),
      wasmController.prepare().then((videoMeta) => {
        ReactDOM
          // @ts-expect-error REACT 18 BINDINGS are missing aaaaa
          .createRoot(document.getElementById("root"))
          .render(
            <EditorContext.make
              videoMeta={videoMeta}
              wasmController={wasmController}
            >
              <Editor />
            </EditorContext.make>
          );
      }),
    ]);
  });
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
