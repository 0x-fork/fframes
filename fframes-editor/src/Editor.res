open Hooks

let last_from_array = arr => arr->Js.Array.get(arr->Array.length - 1)

@genType.as("Editor") @react.component
let make = () => {
  let layout = useEditorLayout()
  let context = EditorContext.useEditorContext()

  <div className="w-screen h-screen bg-gray-900">
    <div className="overflow-auto flex w-full">
      <div
        style={layout.mediaControls->UseEditorLayout.sizeToStyle}
        className="col-span-2  h-full overflow-auto flex flex-col p-6">
        <h1 className="text-2xl font-medium text-white">
          {switch context.videoMeta.name->Js.String.split("::")->last_from_array {
          | Some(name) => React.string(name)
          | _ => React.string("Unknown video")
          }}
        </h1>
      </div>
      <div style={layout.preview->UseEditorLayout.sizeToStyle} className=" bg-black" />
    </div>
    <div
      style={layout.timeLine->UseEditorLayout.sizeToStyle}
      className="shadow-lg w-screen bg-gray-800">
      <Timeline sectionSize=layout.timeLine />
    </div>
  </div>
}
