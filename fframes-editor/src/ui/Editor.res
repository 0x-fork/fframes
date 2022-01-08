open Hooks

@genType.as("Editor") @react.component
let make = () => {
  let layout = useEditorLayout()
  let context = EditorContext.useEditorContext()

  <div className="dark w-screen h-screen bg-gray-900">
    <div className="overflow-auto flex w-full">
      <div
        style={layout.mediaControls->UseEditorLayout.sizeToStyle}
        className="col-span-2  h-full overflow-auto flex flex-col py-6">
        <h1 className="text-2xl mb-2 font-medium text-white px-6">
          {switch context.videoMeta.name->Js.String.split("::")->Utils.Array.last {
          | Some(name) => React.string(name)
          | _ => React.string("Unknown video")
          }}
        </h1>
        <MediaList />
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
