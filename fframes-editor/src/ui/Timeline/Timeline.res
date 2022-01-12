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
let timeline_margin_y = 64.0
let scene_height_size = 120.0

let calcMaxSceneWidth = (size: UseEditorLayout.sectionSize) => size.width -. timeline_margin_x *. 2.

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

  ctx->clipOverTimeLineElement(~y=timeline_margin_y, ~width=maxSceneWidth)
  let maxFramesInScene = (maxSceneWidth /. width)->Js.Math.floor->Float.toInt
  let framesBreak = editorContext.videoMeta.durationInFrames / maxFramesInScene

  Range.forEach(0, maxFramesInScene, i => {
    Js.Console.log((i * framesBreak)->Js.BigInt.fromInt)
    let svg = editorContext.wasmController.render_frame((i * framesBreak)->Js.BigInt.fromInt)
    let image = Image.make(~width, ~height=scene_height_size)

    image->Image.setSrc(svg->Image.btoa |> Js.String.concat("data:image/svg+xml;base64,"))
    image->Image.onLoad(() => {
      ctx->drawImage(
        ~imageData=image,
        ~dy=timeline_margin_y,
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
  let maxSceneWidth = calcMaxSceneWidth(size)

  ctx->clipOverTimeLineElement(~y=32., ~width=maxSceneWidth)
  ctx->Canvas2d.setFillStyle(String, "#9ca3af")
  ctx->Canvas2d.fillRect(
    ~x=timeline_margin_x /. 2.0,
    ~y=timeline_margin_y,
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

let renderTimeSlots = (
  ctx,
  size: UseEditorLayout.sectionSize,
  editorContext: EditorContext.editorContext,
) => {
  let coordinate_step = 100
  let full_timestamp_each_steps = 2
  let timestamp_slot_size = 40

  let maxSceneWidth = calcMaxSceneWidth(size)
  let stepsCount =
    maxSceneWidth
    ->Utils.Math.divideFloat(coordinate_step->Float.fromInt)
    ->Js.Math.floor
    ->Float.toInt

  let stepDuration = editorContext.videoMeta.durationInFrames / stepsCount

  Range.forEach(0, stepsCount, i => {
    let x = (i * coordinate_step)->Float.fromInt +. timeline_margin_x /. 2.

    ctx->Canvas2d.beginPath
    ctx->Canvas2d.moveTo(~x, ~y=0.)
    ctx->Canvas2d.lineTo(~x, ~y=18.)

    ctx->Canvas2d.setStrokeStyle(String, "#475569")
    ctx->Canvas2d.stroke

    if mod(i, full_timestamp_each_steps) === 0 {
      ctx->Canvas2d.font("12px sans-serif")
      ctx->Canvas2d.setFillStyle(String, "#64748b")

      (i * stepDuration)
      ->Float.fromInt
      ->Utils.Math.divideFloat(editorContext.videoMeta.fps->Float.fromInt)
      ->Utils.Duration.formatSeconds
      ->Canvas2d.fillText(ctx, ~x=x +. 8., ~y=14.)
    }
  })
}

@react.component
let make = (~sectionSize: UseEditorLayout.sectionSize) => {
  let canvasRef = React.useRef(Js.Nullable.null)
  let editorContext = EditorContext.useEditorContext()

  React.useEffect1(() => {
    canvasRef.current
    ->Js.Nullable.toOption
    ->Belt.Option.map(element => {
      let context = Webapi.Canvas.CanvasElement.getContext2d(element)
      let scale = Web.Window.devicePixelRatio

      let scaledSize: UseEditorLayout.sectionSize = {
        width: sectionSize.width *. scale,
        height: sectionSize.height *. scale,
        scale: scale,
      }

      (element->Web.Element.style)["height"] = `${sectionSize.height->Float.toString}px`
      (element->Web.Element.style)["width"] = `${sectionSize.width->Float.toString}px`

      element->Canvas.CanvasElement.setHeight(scaledSize.height->Js.Math.floor->Float.toInt)
      element->Canvas.CanvasElement.setWidth(scaledSize.width->Js.Math.floor->Float.toInt)

      context->Canvas2d.scale(~x=scale, ~y=scale)
      context->renderTimeSlots(sectionSize, editorContext)

      switch editorContext.editorState.playState {
      | CantPlay => context->renderScenesPlaceholder(sectionSize, editorContext)
      | _ => context->renderMainScene(sectionSize, ~editorContext)
      }

      ()
    })
    ->ignore

    None
  }, [sectionSize])

  <canvas
    width={`${sectionSize.width->Js.Math.floor->Float.toString}px`}
    height={`${sectionSize.height->Js.Math.floor->Float.toString}px`}
    ref={ReactDOM.Ref.domRef(canvasRef)}
  />
}
