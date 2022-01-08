open Belt

type editorState = {frame: int}

type action = NewFrame(int)

let editorReducer = (state, action) => {
  switch action {
  | NewFrame(number) => {frame: number}
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

  @react.component @genType
  let make = (~wasmController, ~videoMeta, ~children) => {
    let (editorState, dispatch) = React.useReducer(
      editorReducer,
      {
        frame: 0,
      },
    )

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
