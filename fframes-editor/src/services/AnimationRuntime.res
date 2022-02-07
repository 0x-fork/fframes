open Belt

/* @returns whether continue execution or not */
type onFrame = (~secondsFromStart: float) => bool

module AudioRuntime = {
  let rafId: ref<option<Webapi.rafId>> = ref(None)
  let audioContext = ref(None)
  let startTime = ref(0.)

  let rec frame = (~onFrame: onFrame, timestamp) => {
    let secondsFromStart =
      audioContext.contents->Utils.Option.unwrap->WebAudio.Context.getCurrentTime -.
        startTime.contents

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
      let source = ctx->WebAudio.Context.createBufferSource
      source->WebAudio.Node.setBuffer(audioInfo.audioData)

      source
    })
  }

  let startAnimation = (~onFrame, ~videoMeta) => {
    open! WebAudio

    let ctx = Context.create()
    audioContext := Some(ctx)

    let playingSources = ctx->connectCurrentlyPlayingAudio(0, videoMeta)

    let gain = ctx->Context.createGain
    gain["gain"]["value"] = 0.4

    gain->Node.connect(ctx.destination)->ignore
    startTime := ctx->Context.getCurrentTime

    playingSources->Array.forEach(source => {
      source->Node.connect(gain)
      source->Node.start(startTime.contents)
    })

    Webapi.requestAnimationFrame(frame(~onFrame))
  }
}
