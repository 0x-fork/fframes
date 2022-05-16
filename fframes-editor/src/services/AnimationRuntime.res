open WebAudio
open Belt

/* @returns whether continue execution or not */
type onFrame = (~secondsFromStart: float) => bool

module AudioRuntime = {
  let rafId: ref<option<Webapi.rafId>> = ref(None)
  let playingSources: ref<array<(string, AudioNode.t)>> = ref([])
  let audioContext = ref(None)
  let startTime = ref(0.)

  let rec frame = (~onFrame: onFrame, _timestamp) => {
    let secondsFromStart =
      audioContext.contents->Utils.Option.unwrap->AudioContext.getCurrentTime -. startTime.contents

    if onFrame(~secondsFromStart) {
      rafId := Some(Webapi.requestCancellableAnimationFrame(frame(~onFrame)))
    }
  }

  let connectAudioFiles = (ctx, videoMeta: WasmController.videoMeta) => {
    let {mediaList} = MediaLoader.MediaLoaderObserver.get()
    videoMeta.audioMap
    ->Utils.Option.unwrap
    ->Js.Dict.keysToArray
    ->Array.keepMap(audioName => {
      mediaList
      ->Map.String.getExn(audioName)
      ->(
        (media: MediaLoader.loadableMedia) =>
          switch media {
          | Media(media) =>
            switch media {
            | Audio(info) => Some((audioName, info))
            | _ => None
            }
          | _ => None
          }
      )
    })
    ->Array.map(res => {
      let (name, info) = res
      let source = ctx->AudioContext.createBufferSource
      source->AudioNode.setBuffer(info.audioData)

      (name, source)
    })
  }

  let startAnimation = (~onFrame, ~currentFrame, ~videoMeta) => {
    let ctx = AudioContext.create()
    audioContext := Some(ctx)

    playingSources := ctx->connectAudioFiles(videoMeta)

    let gain = ctx->AudioContext.createGain
    gain["gain"]["value"] = 0.2

    gain->AudioNode.connect(ctx.destination)->ignore
    startTime := ctx.currentTime

    videoMeta.audioMap->Belt.Option.forEach(audioMap => {
      playingSources.contents->Array.forEach(nameAndSource => {
        let (name, source) = nameAndSource
        let (startFrame, endFrame) = audioMap->Js.Dict.get(name)->Utils.Option.unwrap

        let offset = (currentFrame - startFrame)->Float.fromInt /. videoMeta.fps->Float.fromInt
        let duration = if startFrame > currentFrame {
          (endFrame - startFrame - currentFrame)->Float.fromInt /. videoMeta.fps->Float.fromInt
        } else {
          (endFrame - currentFrame)->Float.fromInt /. videoMeta.fps->Float.fromInt
        }->Js.Math.max(0.)
        source->AudioNode.connect(gain)

        if offset < 0. {
          source->AudioNode.startWithOffset(
            ~startTime=startTime.contents +. Js.Math.abs(offset),
            ~offset=0.,
            ~duration,
          )
        } else {
          source->AudioNode.startWithOffset(~startTime=startTime.contents, ~offset, ~duration)
        }
      })
    })

    Webapi.requestAnimationFrame(frame(~onFrame))
  }

  let stop = () => {
    rafId.contents->Belt.Option.map(Webapi.cancelAnimationFrame)->ignore
    playingSources.contents->Array.forEach(nameAndSource => {
      let (_, source) = nameAndSource
      source->AudioNode.stop
    })
  }
}
