open Belt
open CanvasSize

@react.component
let make = (~sectionSize: UseEditorLayout.sectionSize) => {
  let editorContext = EditorContext.useEditorContext()

  let size = React.useMemo4(() => {
    let scale = Web.Window.devicePixelRatio
    let maxSceneWidth = sectionSize.width -. timeline_margin_x->Float.fromInt

    {
      width: sectionSize.width,
      height: sectionSize.height,
      scaledWidth: sectionSize.width *. scale,
      scaledHeight: sectionSize.height *. scale,
      scale: scale,
      maxSceneWidth: maxSceneWidth,
      frameToPxRatio: maxSceneWidth /. editorContext.videoMeta.durationInFrames->Float.fromInt,
    }
  }, (
    sectionSize.height,
    sectionSize.width,
    sectionSize.scale,
    editorContext.videoMeta.durationInFrames,
  ))

  <div className="relative"> <SceneMapCanvas size /> <SeekBarCanvas size /> </div>
}
