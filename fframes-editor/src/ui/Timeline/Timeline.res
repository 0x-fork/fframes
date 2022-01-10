open Belt
module Canvas = Webapi.Canvas
module Canvas2d = Webapi.Canvas.Canvas2d

@send
external drawImage: (
  Canvas.Canvas2d.t,
  ~imageData: Image.t,
  ~dx: float,
  ~dy: float,
  ~dirtyWidth: float,
  ~dirtyHeight: float,
) => unit = "drawImage"

let timeline_margin_x = 64.0
let scene_height_size = 120.0

let renderRoundedRect = (ctx, ~x, ~y, ~width, ~height, ~radius, ()) => {
  ctx->Canvas2d.beginPath
  ctx->Canvas2d.moveTo(~x=x +. radius, ~y)

  ctx->Canvas2d.arcTo(~x1=x +. width, ~y1=y, ~x2=x +. width, ~y2=y +. height, ~r=radius)
  ctx->Canvas2d.arcTo(~x1=x +. width, ~y1=y +. height, ~x2=x, ~y2=y +. height, ~r=radius)
  ctx->Canvas2d.arcTo(~x1=x, ~y1=y +. height, ~x2=x, ~y2=y, ~r=radius)
  ctx->Canvas2d.arcTo(~x1=x, ~y1=y, ~x2=x +. width, ~y2=y, ~r=radius)

  ctx->Canvas2d.stroke
}

let clipOverTimeLineElement = (ctx, ~y, ~width) => {
  ctx->renderRoundedRect(
    ~x=timeline_margin_x /. 2.0,
    ~y,
    ~width,
    ~height=scene_height_size,
    ~radius=8.0,
    (),
  )
  ctx->Canvas2d.clip
}

let renderMainScene = (
  ctx,
  size: UseEditorLayout.sectionSize,
  ~editorContext: EditorContext.editorContext,
) => {
  let maxSceneWidth = size.width -. timeline_margin_x *. 2.
  let aspectRatio =
    editorContext.videoMeta.width->Float.fromInt /. editorContext.videoMeta.height->Float.fromInt

  let width = scene_height_size *. aspectRatio

  ctx->clipOverTimeLineElement(~y=32.0, ~width=maxSceneWidth)
  let maxFramesInScene = (maxSceneWidth /. width)->Js.Math.floor->Float.toInt
  let framesBreak = 150 // editorContext.videoMeta.durationInFrames / maxFramesInScene

  Range.forEach(0, maxFramesInScene, i => {
    %debugger;
    let svg = editorContext.wasmController.render_frame((i * framesBreak)->Js.BigInt.fromInt)
    let image = Image.make(~width, ~height=scene_height_size)

    image->Image.setSrc(svg->Image.btoa |> Js.String.concat("data:image/svg+xml;base64,"))
    image->Image.onLoad(() => {
      ctx->drawImage(
        ~imageData=image,
        ~dy=32.,
        ~dx=timeline_margin_x /. 2. +. i->Js.Float.fromInt *. width,
        ~dirtyHeight=scene_height_size,
        ~dirtyWidth=width,
      )
    })
  })

  ()
}

let renderScenesPlaceholder = (
  ctx,
  size: UseEditorLayout.sectionSize,
  editorContext: EditorContext.editorContext,
) => {
  let maxSceneWidth = size.width -. timeline_margin_x *. 2.

  ctx->clipOverTimeLineElement(~y=32., ~width=maxSceneWidth)
  ctx->Canvas2d.setFillStyle(String, "#9ca3af")
  ctx->Canvas2d.fillRect(
    ~x=timeline_margin_x /. 2.0,
    ~y=32.0,
    ~w=maxSceneWidth,
    ~h=scene_height_size,
  )

  // let sceneHeight
}

let renderCanvas = (element, context) => {
  let width = float_of_int(Canvas.CanvasElement.width(element))
  let height = float_of_int(Canvas.CanvasElement.height(element))
  let centerX = width /. 2.0
  let centerY = height /. 2.0

  Js.Console.log2(width, height)
}

@react.component
let make = (~sectionSize: UseEditorLayout.sectionSize) => {
  let canvasRef = React.useRef(Js.Nullable.null)
  let editorContext = EditorContext.useEditorContext()

  React.useEffect0(() => {
    switch editorContext.editorState.playState {
    | CantPlay =>
      canvasRef.current
      ->Js.Nullable.toOption
      ->Belt.Option.forEach(element => {
        let context = Webapi.Canvas.CanvasElement.getContext2d(element)
        context->renderScenesPlaceholder(sectionSize, editorContext)
      })
    | _ =>
      canvasRef.current
      ->Js.Nullable.toOption
      ->Belt.Option.forEach(element => {
        let context = Webapi.Canvas.CanvasElement.getContext2d(element)
        context->renderMainScene(sectionSize, ~editorContext)
      })
    }
    None
  })

  <canvas
    width={`${sectionSize.width->Float.toString}px`}
    height={`${sectionSize.height->Float.toString}px`}
    ref={ReactDOM.Ref.domRef(canvasRef)}
  />
}
