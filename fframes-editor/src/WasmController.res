@genType.as("VideoMeta")
type videoMeta = {
  name: string,
  width: int,
  height: int,
  fps: int,
  durationInFrames: int,
  audioMap: Js.Nullable.t<Js.Dict.t<(int, int)>>,
}

@genType.as("WasmController")
type t = {
  add_audio_source: (string, ReScriptJs.Js.Float32Array.t) => unit,
  add_image_source: (string, string) => unit,
  add_subtitles_source: (string, string) => int,
  default: unit => Js.Promise.t<unit>,
  prepare: unit => Js.Promise.t<videoMeta>,
  render_frame: Js.BigInt.t => string,
}

module type WasmBridge = {
  let videoMeta: videoMeta
  let controller: t
}

external getFrame: (t, Js.BigInt.t) => string = "getFrame"
