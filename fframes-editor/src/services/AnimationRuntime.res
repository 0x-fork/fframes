open Belt

/* @returns whether continue execution or not */
type onFrame = (~msDelta: float, ~fps: int) => unit

module RafAnimation = {
  let rafId: ref<option<Webapi.rafId>> = ref(None)

  let rec frame = (~onFrame: onFrame, ~start, timestamp) => {
    let msDelta = timestamp -. start

    onFrame(~msDelta, ~fps=(1000. /. msDelta)->Float.toInt)

    rafId := Some(Webapi.requestCancellableAnimationFrame(frame(~onFrame, ~start=timestamp)))
  }

  let stop = () => {
    rafId.contents->Belt.Option.map(Webapi.cancelAnimationFrame)->ignore
  }

  let startAnimation = (~onFrame) => {
    Webapi.requestAnimationFrame(start => {
      Webapi.requestAnimationFrame(frame(~onFrame, ~start))
      ()
    })
  }
}
