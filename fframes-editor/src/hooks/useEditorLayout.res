open UseDimensions

let min_timeline_height = 300
let min_media_controls_width = 370

type sectionSize = {
  height: float,
  width: float,
  scale: float,
}

let sizeToStyle = ({width, height, scale}: sectionSize) => {
  ReactDOMStyle.make(
    ~width=`${width->Belt.Float.toString}px`,
    ~height=`${height->Belt.Float.toString}px`,
    (),
  )
}

type editorLayout = {
  timeLine: sectionSize,
  preview: sectionSize,
  mediaControls: sectionSize,
}

let calculatePreviewSize = (
  windowDimensions: dimensions,
  {width, height}: WasmController.videoMeta,
) => {
  let max_preview_width = windowDimensions.width - min_media_controls_width
  let max_preview_height = windowDimensions.height - min_timeline_height

  if width > max_preview_width || height > max_preview_height {
    let scale = min(
      max_preview_width->float_of_int /. width->float_of_int,
      max_preview_height->float_of_int /. height->float_of_int,
    )

    {
      width: width->Belt.Int.toFloat *. scale,
      height: height->Belt.Int.toFloat *. scale,
      scale: scale,
    }
  } else {
    {
      scale: 1.0,
      width: width->Belt.Int.toFloat,
      height: height->Belt.Int.toFloat,
    }
  }
}

let useEditorLayout = () => {
  let viewportSize = useDimensions()
  let {videoMeta} = EditorContext.useEditorContext()

  viewportSize
  ->calculatePreviewSize(videoMeta)
  ->(
    previewSize => {
      preview: previewSize,
      mediaControls: {
        width: viewportSize.width->Belt.Int.toFloat -. previewSize.width,
        height: previewSize.height,
        scale: 1.0,
      },
      timeLine: {
        height: viewportSize.height->Belt.Int.toFloat -. previewSize.height,
        width: viewportSize.width->Belt.Int.toFloat,
        scale: 1.0,
      },
    }
  )
}
