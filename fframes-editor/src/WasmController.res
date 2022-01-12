@genType.as("VideoMeta")
type videoMeta = {
  name: string,
  width: int,
  height: int,
  fps: int,
  durationInFrames: int,
}

@genType.as("WasmController")
type t = {
  add_audio_source: (string, ReScriptJs.Js.Float32Array.t) => unit,
  add_subtitles_source: (string, string) => unit,
  default: unit => Js.Promise.t<unit>,
  prepare: unit => Js.Promise.t<videoMeta>,
  render_frame: Js.BigInt.t => string,
}

external getFrame: (t, Js.BigInt.t) => string = "getFrame"
