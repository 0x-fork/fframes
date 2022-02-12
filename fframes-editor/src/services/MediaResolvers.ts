import { MediaResolver, resolveMedia } from "./mediaLoader.gen";
import { createDecoder } from "minimp3-wasm/dist/minimp3-wasm";
// @ts-expect-error no  types
import minimp3decoderWasm from "minimp3-wasm/dist/decoder.opt.wasm?url";

const audioContext = new AudioContext();

export const resolveAudio: MediaResolver = async (
  name,
  url,
  wasmController
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

  wasmController.add_audio_source(name, monoPcm);
  const audioData = await audioContext.decodeAudioData(arrayBuffer);

  return resolveMedia(name, {
    tag: "Audio",
    value: {
      arrayBuffer,
      audioData,
      monoPcmData: monoPcm,
      sampleRate: data.samplingRate,
      duration: monoPcm.length / data.samplingRate,
    },
  });
};

export const resolveSubtitles: MediaResolver = async (
  name,
  url,
  wasmController
) => {
  const response = await fetch(url);
  const text = await response.text();
  const phrasesCount = wasmController.add_subtitles_source(name, text);

  return resolveMedia(name, {
    tag: "Subtitles",
    value: phrasesCount,
  });
};

export const resolveFont: MediaResolver = async (name, url, wasmController) => {
  const fontName = name.replace(/\.[^/.]+$/, "");
  const fontFace = new FontFace(fontName, `url(${url})`);

  const loadedFont = await fontFace.load();
  document.fonts.add(loadedFont);

  return resolveMedia(name, {
    tag: "Font",
    value: `${loadedFont.weight} (${loadedFont.unicodeRange})`,
  });
};

const loadImage = (url: string) =>
  new Promise<HTMLImageElement>((resolve, reject) => {
    const img = new Image();
    img.addEventListener("load", () => resolve(img));
    img.addEventListener("error", (err) => reject(err));
    img.src = url;
  });

export const resolveImage: MediaResolver = async (
  name,
  url,
  wasmController
) => {
  const image = await loadImage(url);
  wasmController.add_image_source(name, url);

  return resolveMedia(name, {
    tag: "Image",
    value: {
      width: image.naturalWidth,
      height: image.naturalHeight,
      src: url,
    },
  });
};
