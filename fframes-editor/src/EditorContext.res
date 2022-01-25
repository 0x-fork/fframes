open Webapi

type playState = Playing | Paused | WaitingForAction | CantPlay

type editorState = {
  frame: int,
  playState: playState,
  svg: option<string>,
  wasmController: WasmController.t,
}

type newFrameAction = FrameIndex(int) | DeltaTime(float)
type action = NewFrame(newFrameAction) | AllowPlay | Play | Pause

let editorReducer = (state, action) => {
  switch action {
  | NewFrame(indexOrDelta) => {
      let frame = switch indexOrDelta {
      | FrameIndex(index) => index
      | DeltaTime(delta) =>
        (delta *. 60->Belt.Float.fromInt /. 1000.)->Js.Math.round->Belt.Int.fromFloat
      }

      let svg = state.wasmController.render_frame(frame->Js.BigInt.fromInt)
      {...state, frame: frame, svg: Some(svg)}
    }
  | AllowPlay => {...state, playState: WaitingForAction}
  | Play => {...state, playState: Playing}
  | Pause => {...state, playState: Paused}
  }
}

@genType
type editorContext = {
  wasmController: WasmController.t,
  videoMeta: WasmController.videoMeta,
  editorState: editorState,
}

module DocumentEvent = Dom.EventTarget.Impl(Dom.Window)
let editorContext = React.createContext(None)

let useEditorContext = () => {
  let context = React.useContext(editorContext)

  switch context {
  | Some(context) => context
  | _ => failwith("Missing editorContext.")
  }
}

module EditorContext = {
  let providerElement = React.Context.provider(editorContext)

  let getDefaultState = (~wasmController: WasmController.t) => {
    switch MediaLoader.MediaLoaderObserver.get() {
    | state if state.allMediaLoaded => {
        frame: 0,
        playState: WaitingForAction,
        wasmController: wasmController,
        svg: wasmController.render_frame(0->Js.BigInt.fromInt)->Utils.Option.some,
      }
    | _ => {
        frame: 0,
        playState: CantPlay,
        wasmController: wasmController,
        svg: None,
      }
    }
  }

  @react.component @genType
  let make = (
    ~wasmController: WasmController.t,
    ~videoMeta: WasmController.videoMeta,
    ~children,
  ) => {
    let (editorState, dispatch) = React.useReducer(editorReducer, getDefaultState(~wasmController))

    React.useLayoutEffect0(() => {
      Some(
        MediaLoader.MediaLoaderObserver.subscribe(state => {
          if state.allMediaLoaded && editorState.playState === CantPlay {
            dispatch(AllowPlay)

            dispatch(NewFrame(FrameIndex(editorState.frame)))
          }
        }),
      )
    })

    let onFrame = (~msDelta, ~fps) => {
      NewFrame(DeltaTime(msDelta))->dispatch->ignore
    }

    let start = React.useCallback1(() => {
      dispatch(Play)
      AnimationRuntime.RafAnimation.startAnimation(~onFrame)
    }, [dispatch])

    let pause = () => {
      dispatch(Pause)
      AnimationRuntime.RafAnimation.stop()
    }

    React.useEffect1(() => {
      let handleKeydown = e => {
        Js.Console.log(editorState)
        switch e->Dom.KeyboardEvent.key {
        | " " if editorState.playState === Playing => pause()
        | " " => start()
        | _ => ()
        }
      }

      Dom.window
      |> DocumentEvent.asEventTarget
      |> Dom.EventTarget.addKeyDownEventListener(handleKeydown)

      Some(
        () =>
          Dom.window
          |> DocumentEvent.asEventTarget
          |> Dom.EventTarget.removeKeyDownEventListener(handleKeydown),
      )
    }, [editorState])

    React.createElement(
      providerElement,
      {
        "value": Some({
          wasmController: wasmController,
          videoMeta: videoMeta,
          editorState: editorState,
        }),
        "children": children,
      },
    )
  }
}
