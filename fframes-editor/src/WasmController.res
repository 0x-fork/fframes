@genType.as("VideoMeta")
type videoMeta = {
  name: string,
  width: int,
  height: int,
  duration: int,
}

@genType.as("WasmController")
type t = {
  addAudioSource: (string, Js.Float32Array.t) => unit,
  addSubtitlesSource: (string, string) => unit,
  default: unit => Js.Promise.t<unit>,
  prepare: unit => Js.Promise.t<videoMeta>,
}

external getFrame: (t, Js.BigInt.t) => string = "getFrame"

 