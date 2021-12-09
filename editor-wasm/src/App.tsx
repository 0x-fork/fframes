import React, { useEffect } from "react";
import "./App.css";
import init, { render_frame, cache_audio, cache_subtitles } from "../bind/pkg";
import testAudio from "../media/marketing.mp3";
import subtitlesUrl from "../media/subtitles.vtt?url";
import { createDecoder } from "minimp3-wasm/dist/minimp3-wasm";
import minimp3decoderWasm from "minimp3-wasm/dist/decoder.opt.wasm?url";

let audioContext: AudioContext;
let audioBuffer: ArrayBuffer;
let duration: number;
let offset: number;

const FPS = 30;
let state = "idle";

let monoCache = localStorage.getItem("cache");

function App() {
  const ref = React.useRef<HTMLDivElement>(null);

  function frame() {
    if (state === "paused") {
      return;
    }

    console.time("frame");
    const currentFrame = Math.round((audioContext.currentTime - offset) * FPS) + 100;
    const frame_svg = render_frame(BigInt(currentFrame));

    if (frame_svg && ref.current && ref.current.innerHTML !== frame_svg) {
      ref.current.innerHTML = frame_svg;
    }

    console.timeEnd("frame");

    requestAnimationFrame(() => frame());
  }

  async function start() {
    audioContext = new AudioContext();
    const source = audioContext.createBufferSource();
    offset = audioContext.currentTime;

    var gainNode = audioContext.createGain();
    gainNode.gain.value = 0.0;
    gainNode.connect(audioContext.destination);

    source.connect(gainNode);

    fetch(subtitlesUrl)
      .then((res) => res.text())
      .then((rawSubtitles) => {
        cache_subtitles("subtitles", rawSubtitles);
      });

    audioContext
      .decodeAudioData(audioBuffer.slice(0))
      .then(async (audioData) => {
        const decoder = await createDecoder(
          new Uint8Array(audioBuffer.slice(0)),
          minimp3decoderWasm
        );
        duration = audioData.duration;
        const data = await decoder.decode();
        const length = Math.floor(data.pcm.length / data.numChannels);

        const mono = new Float32Array(length);
        for (let i = 0, j = 0; i < length; i += 1, j += data.numChannels) {
          mono[i] = data.pcm[j]; // (+data.pcm[j + 1]) / 2;
        }

        cache_audio("marketing", mono);
        source.buffer = audioData;
        offset = audioContext.currentTime - offset;
        source.start(audioContext.currentTime);
        requestAnimationFrame(() => {
          frame();
        });
      });
  }

  useEffect(() => {
    document.addEventListener("keydown", (e) => {
      if (e.key === " ") {
        e.preventDefault();
        e.stopPropagation();

        if (state === "idle") {
          state = "play";
          start();
        } else if (state === "paused") {
          audioContext.resume();
          state = "play";
          frame();
        } else if (state === "play") {
          audioContext.suspend();
          state = "paused";
        }
      }
    });

    init().then(() => {
      fetch(testAudio)
        .then((res) => res.arrayBuffer())
        .then((arrayBuffer) => {
          audioBuffer = arrayBuffer;

          console.log(monoCache);
          if (monoCache) {
            cache_audio("test", new Float32Array(JSON.parse(monoCache)));
            render_frame(BigInt(0));
          }
        });
      // .then(async (uint8Buffer) => {
      //

      //   return decoder.decode();
      // })
      // .then((decode) => decode.pcm.filter((_, i) => i % 2 !== 0))
      // .then((samples) => {
      //   cache_audio("test", new Float32Array(samples));
      // });
    });
  }, []);

  return (
    <>
      <div
        style={{
          height: "1080px",
          width: "1920px",
          border: "4px solid black",
        }}
        className="App"
        ref={ref}
      ></div>
    </>
  );
}

export default App;
