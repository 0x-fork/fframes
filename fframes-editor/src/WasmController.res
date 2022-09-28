type audioTrack = {
  name: string, 
  start: int,
  end: int,
}

@genType.as("VideoMeta")
type videoMeta = {
  name: string,
  width: int,
  height: int,
  fps: int,
  durationInFrames: int,
  audioMap: Js.Nullable.t<array<audioTrack>>
}

@genType.as("WasmController")
type t = {
  add_audio_source: (string, ReScriptJs.Js.Int16Array.t) => unit,
  add_image_source: (string, string, Js.Nullable.t<string>) => unit,
  add_subtitles_source: (string, string) => int,
  default: unit => Js.Promise.t<unit>,
  prepare: unit => Js.Promise.t<videoMeta>,
  render_frame: Js.BigInt.t => string,
  render_preview_frame: Js.BigInt.t => string,
  get_font_file_family: Js.Uint8Array.t => Js.Nullable.t<Js.Uint8Array.t>,
}

module type WasmBridge = {
  let videoMeta: videoMeta
  let controller: t
}

external getFrame: (t, Js.BigInt.t) => string = "getFrame"
