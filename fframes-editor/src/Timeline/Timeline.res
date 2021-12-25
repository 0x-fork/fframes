open Belt
module Canvas = Webapi.Canvas
module Canvas2d = Webapi.Canvas.Canvas2d

let timeline_margin_x = 128.0
let scene_height_size = 120.0

let renderRoundedRect = (ctx, ~x, ~y, ~width, ~height, ~radius, ()) => {
  ctx->Canvas2d.beginPath
  ctx->Canvas2d.moveTo(~x=x +. radius, ~y)

  ctx->Canvas2d.arcTo(~x1=x +. width, ~y1=y, ~x2=x +. width, ~y2=y +. height, ~r=radius)
  ctx->Canvas2d.arcTo(~x1=x +. width, ~y1=y +. height, ~x2=x, ~y2=y +. height, ~r=radius)
  ctx->Canvas2d.arcTo(~x1=x, ~y1=y +. height, ~x2=x, ~y2=y, ~r=radius)
  ctx->Canvas2d.arcTo(~x1=x, ~y1=y, ~x2=x +. width, ~y2=y, ~r=radius)
  // ctx->Canvas2d.lineTo(~x=x +. width -. radius, ~y)
  // ctx->Canvas2d.lineTo(~x=x +. width -. radius, ~y)
  // ctx->Canvas2d.quadraticCurveTo(~cp1x=x +. width, ~cp1y=y, ~x=x +. width, ~y=y +. radius)

  ctx->Canvas2d.stroke
}

let renderScenesPlaceholder = (
  ctx,
  size: UseEditorLayout.sectionSize,
  editorContext: EditorContext.editorContext,
) => {
  let maxSceneWidth = size.width -. timeline_margin_x

  ctx->renderRoundedRect(
    ~x=timeline_margin_x /. 2.0,
    ~y=32.0,
    ~width=maxSceneWidth,
    ~height=scene_height_size,
    ~radius=8.0,
    (),
  )
  ctx->Canvas2d.clip

  // ctx->Canvas2d.clip
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
    // %debugger
    canvasRef.current
    ->Js.Nullable.toOption
    ->Belt.Option.forEach(element => {
      let context = Webapi.Canvas.CanvasElement.getContext2d(element)
      context->renderScenesPlaceholder(sectionSize, editorContext)
    })

    None
  })

  <canvas
    width={`${sectionSize.width->Float.toString}px`}
    height={`${sectionSize.height->Float.toString}px`}
    ref={ReactDOM.Ref.domRef(canvasRef)}
  />
}
