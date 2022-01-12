type playState = Playing | Paused | WaitingForAction | CantPlay

type editorState = {frame: int, playState: playState, svg: option<string>}

type action = NewFrame(int, string) | AllowPlay

let editorReducer = (state, action) => {
  switch action {
  | NewFrame(frame, svg) => {...state, frame: frame, svg: Some(svg)}
  | AllowPlay => {...state, playState: WaitingForAction}
  }
}

@genType
type editorContext = {
  wasmController: WasmController.t,
  videoMeta: WasmController.videoMeta,
  editorState: editorState,
}

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
        svg: wasmController.render_frame(1240->Js.BigInt.fromInt)->Utils.Option.some,
      }
    | _ => {
        frame: 0,
        playState: CantPlay,
        svg: None,
      }
    }
  }

  @react.component @genType
  let make = (~wasmController: WasmController.t, ~videoMeta, ~children) => {
    let (editorState, dispatch) = React.useReducer(editorReducer, getDefaultState(~wasmController))

    React.useLayoutEffect0(() => {
      Some(
        MediaLoader.MediaLoaderObserver.subscribe(state => {
          if state.allMediaLoaded && editorState.playState === CantPlay {
            dispatch(AllowPlay)

            let previewSvg = wasmController.render_frame(editorState.frame->Js.BigInt.fromInt)
            dispatch(NewFrame(editorState.frame, previewSvg))
          }
        }),
      )
    })

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
