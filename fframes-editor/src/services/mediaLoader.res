type audioInfo = {
  duration: int,
  sampleRate: int,
  arrayBuffer: Js.ArrayBuffer.t,
}

type imageInfo = {
  width: int,
  height: int,
}

@genType
type processedMedia =
  | Font(string)
  | Subtitles(string)
  | Image(string, imageInfo)
  | Audio((string, audioInfo))

type imageImport = {default: string}

@genType.as("MediaResolver")
type mediaResolveFn = (~name: string, ~url: string, WasmController.t) => Js.Promise.t<processedMedia>

@module("./MediaResolvers") external resolveAudio: mediaResolveFn = "resolveAudio"
@module("./MediaResolvers") external resolveSubtitles: mediaResolveFn = "resolveSubtitles"

let processAudio = (controller: WasmController.t, imports: Js.Dict.t<imageImport>) => {
  imports
  ->Js.Dict.toArray
  ->Belt.Array.keepMap(((name, moduleVal)) =>
    switch name {
    | name if name->Js.String.endsWith(".mp3") => Some(resolveAudio(~name))
    | name if name->Js.String.endsWith(".vtt") => Some(resolveSubtitles)
    | _ => None
    }
  )
  // ->Belt.Array.map(resolveFn => resolveFn(controller, name, moduleVal.default))
  // ->Js.Promise.all
}
