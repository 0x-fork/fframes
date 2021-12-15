type editorContext = {wasmController: WasmController.t}

let editorContext = React.createContext(None)

module EditorContextProvider = {
  let providerElement = React.Context.provider(editorContext)

  @react.component
  let make = (~wasmController: WasmController.t, ~children) => {
    React.createElement(
      providerElement,
      {
        "value": Some({
          wasmController
        }),
        "children": children,
      },
    )
  }
}
