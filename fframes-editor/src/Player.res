open Belt

type playState = Playing | Paused | WaitingForAction | CantPlay

@genType
type state = {
  frame: int,
  startPlayingFrame: int,
  playState: playState,
  svg: option<string>,
}

@genType
type action = NewFrame(int) | AllowPlay | Play | Pause

/// This state should only contain a state that changes or affect the animation runtime and will likely change 60 t/s
module MakePlayer = (Wasm: WasmController.WasmBridge) => {
  module PlayerState = {
    type t = state

    let initial = switch MediaLoader.MediaLoaderObserver.get() {
    | state if state.allMediaLoaded => {
        frame: 0,
        startPlayingFrame: 0,
        playState: WaitingForAction,
        svg: Wasm.controller.render_frame(0->Js.BigInt.fromInt)->Utils.Option.some,
      }
    | _ => {
        frame: 0,
        startPlayingFrame: 0,
        playState: CantPlay,
        svg: None,
      }
    }
  }

  include UseObservable.Pubsub(PlayerState)

  let reducer = action => {
    let state = get()

    switch action {
    | NewFrame(frame) if frame == Wasm.videoMeta.durationInFrames => {
        let svg = Wasm.controller.render_frame(frame->Js.BigInt.fromInt)

        {...state, frame: frame, svg: Some(svg), playState: Paused}
      }
    | NewFrame(frame) => {
        let svg = Wasm.controller.render_frame(frame->Js.BigInt.fromInt)

        {...state, frame: frame, svg: Some(svg)}
      }
    | AllowPlay => {...state, playState: WaitingForAction}
    | Play if state.frame >= Wasm.videoMeta.durationInFrames => {
        ...state,
        frame: 0,
        playState: Playing,
      }
    | Play => {...state, playState: Playing, startPlayingFrame: state.frame}
    | Pause => {...state, playState: Paused}
    }
  }

  let sideEffect = (action, dispatch) => {
    switch action {
    | Play if get().playState !== Playing => {
        let onFrame = (~secondsFromStart) => {
          let nextFrame =
            secondsFromStart *. Wasm.videoMeta.fps->Float.fromInt +.
              get().startPlayingFrame->Float.fromInt

          NewFrame(nextFrame->Utils.Math.floor)->dispatch
          get().playState === Playing
        }

        AnimationRuntime.AudioRuntime.startAnimation(
          ~onFrame,
          ~currentFrame=get().frame,
          ~videoMeta=Wasm.videoMeta,
        )
        ()
      }
    | Pause => AnimationRuntime.AudioRuntime.stop()
    | _ => ()
    }
  }

  let rec dispatch = action => {
    sideEffect(action, dispatch)
    reducer(action)->set
  }
}
