open Hooks

@genType
let a = Js.Dict.empty

@genType.as("Editor") @react.component
let make = () => {
  let layout = useEditorLayout()

  let context = EditorContext.useEditorContext()
  let (player, _) = context.usePlayer()

  let videoTitle = React.useMemo1(() => {
    switch context.videoMeta.name->Js.String.split("::")->Utils.Array.last {
    | Some(name) => React.string(name)
    | _ => React.string("Unknown video")
    }
  }, [context.videoMeta])

  <div className="w-screen h-screen bg-gray-900">
    <ReactHelmet>
      <title> {videoTitle} </title>
      <style type_="text/css">
        {React.string(
          `
            #editor-preview > svg {
              transform-origin: top left !important;
              transform: scale(${layout.preview.scale->Js.Float.toString}) !important
            }
          `,
        )}
      </style>
    </ReactHelmet>
    <div className="overflow-auto flex w-full">
      // MediaList
      <div
        style={layout.mediaControls->UseEditorLayout.sizeToStyle}
        className="col-span-2 h-full overflow-auto flex flex-col py-6 border-r border-gray-800">
        <h1 className="text-2xl mb-6 font-medium text-white px-6"> {videoTitle} </h1> <MediaList />
      </div>
      // Preview
      <div
        id="editor-preview"
        style={layout.preview->UseEditorLayout.sizeToStyle}
        className=" bg-black"
        dangerouslySetInnerHTML={{
          "__html": player.svg->Utils.Option.unwrapOr(""),
        }}
      />
    </div>
    <div
      style={layout.timeLine->UseEditorLayout.sizeToStyle}
      className="shadow-lg w-screen bg-gray-800">
      <Timeline sectionSize=layout.timeLine />
    </div>
    <Dock />
  </div>
}
