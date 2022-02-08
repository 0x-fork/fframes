open Belt
open WebAudio

/* @returns whether continue execution or not */
type onFrame = (~secondsFromStart: float) => bool

module AudioRuntime = {
  let rafId: ref<option<Webapi.rafId>> = ref(None)
  let audioContext = ref(None)
  let startTime = ref(0.)

  let rec frame = (~onFrame: onFrame, timestamp) => {
    let secondsFromStart =
      audioContext.contents->Utils.Option.unwrap->AudioContext.getCurrentTime -. startTime.contents

    if onFrame(~secondsFromStart) {
      rafId := Some(Webapi.requestCancellableAnimationFrame(frame(~onFrame)))
    }
  }

  let stop = () => {
    rafId.contents->Belt.Option.map(Webapi.cancelAnimationFrame)->ignore
  }

  let connectCurrentlyPlayingAudio = (ctx, frame, videoMeta: WasmController.videoMeta) => {
    let {mediaList} = MediaLoader.MediaLoaderObserver.get()

    videoMeta.audioMap
    ->Utils.Option.unwrap
    ->Js.Dict.keysToArray
    ->Array.keepMap(audioName => {
      let value =
        videoMeta.audioMap->Utils.Option.unwrap->Js.Dict.get(audioName)->Utils.Option.unwrap

      switch value {
      | (start, end) if frame >= start && frame <= end =>
        mediaList
        ->Map.String.getExn(audioName)
        ->(
          (media: MediaLoader.loadableMedia) =>
            switch media {
            | Media(media) =>
              switch media {
              | Audio(info) => Some(info)
              | _ => None
              }
            | _ => None
            }
        )

      | _ => None
      }
    })
    ->Array.map(audioInfo => {
      let source = ctx->AudioContext.createBufferSource
      source->AudioNode.setBuffer(audioInfo.audioData)

      source
    })
  }

  let startAnimation = (~onFrame, ~videoMeta) => {
    let ctx = AudioContext.create()
    audioContext := Some(ctx)

    let playingSources = ctx->connectCurrentlyPlayingAudio(0, videoMeta)

    let gain = ctx->AudioContext.createGain
    gain["gain"]["value"] = 0.4

    gain->AudioNode.connect(ctx.destination)->ignore
    startTime := ctx.currentTime

    playingSources->Array.forEach(source => {
      source->AudioNode.connect(gain)
      source->AudioNode.start(startTime.contents)
    })

    Webapi.requestAnimationFrame(frame(~onFrame))
  }
}
