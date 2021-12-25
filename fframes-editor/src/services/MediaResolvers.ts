import type { WasmController } from "../WasmController.gen";
import type { processedMedia } from "./MediaLoader.gen";
import { createDecoder } from "minimp3-wasm/dist/minimp3-wasm";
import minimp3decoderWasm from "minimp3-wasm/dist/decoder.opt.wasm?url";

type MediaResolver = (
  wasmController: WasmController,
  name: string,
  url: string
) => Promise<processedMedia>;

export const resolveAudio: MediaResolver = async (
  wasmController,
  name,
  url
) => {
  const response = await fetch(url);
  const arrayBuffer = await response.arrayBuffer();

  const decoder = await createDecoder(
    new Uint8Array(arrayBuffer.slice(0)),
    minimp3decoderWasm
  );

  const data = await decoder.decode();
  const length = Math.floor(data.pcm.length / data.numChannels);

  const monoPcm = new Float32Array(length);
  for (let i = 0, j = 0; i < length; i += 1, j += data.numChannels) {
    monoPcm[i] = data.pcm[j]; // or maybe we should do (data.pcm[j + 1]) / 2?
  }

  wasmController.addAudioSource(name, monoPcm);

  return {
    tag: "Audio",
    value: [
      name,
      {
        arrayBuffer,
        sampleRate: data.samplingRate,
        duration: length / data.numSamples,
      },
    ],
  };
};

export const resolveSubtitles: MediaResolver = async (
  wasmController,
  name,
  url
) => {
  const response = await fetch(url);
  const text = await response.text();

  wasmController.addSubtitlesSource(name, text);

  return {
    tag: "Subtitles",
    value: name,
  };
};


