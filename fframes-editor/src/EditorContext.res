@genType
type editorContext = {wasmController: WasmController.t, videoMeta: WasmController.videoMeta}

let editorContext = React.createContext(None)

let useEditorContext = () => {
  let context = React.useContext(editorContext)

  switch context {
  | Some(context) => context
  | _ => failwith("Missing editorContext.")
  }
}

type editorState =  {
  mediaState 
}

module EditorContext = {
  let providerElement = React.Context.provider(editorContext)

  @react.component @genType
  let make = (~wasmController, ~videoMeta, ~children) => {
    React.createElement(
      providerElement,
      {
        "value": Some({
          wasmController: wasmController,
          videoMeta: videoMeta,
        }),
        "children": children,
      },
    )
  }
}
