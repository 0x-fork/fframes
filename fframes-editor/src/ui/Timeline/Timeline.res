open Belt
module Canvas = Webapi.Canvas
module Canvas2d = Webapi.Canvas.Canvas2d

type canvasSize = {
  width: float,
  height: float,
  scale: float,
  scaledWidth: float,
  scaledHeight: float,
  maxSceneWidth: float,
  frameToPxRatio: float,
}

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

let renderMainScene = (ctx, size, editorContext: EditorContext.editorContext) => {
  let aspectRatio =
    editorContext.videoMeta.width->Float.fromInt /. editorContext.videoMeta.height->Float.fromInt
  let width = scene_height_size *. aspectRatio

  ctx->clipOverTimeLineElement(~y=timeline_margin_y, ~width=size.maxSceneWidth)
  let maxFramesInScene = (size.maxSceneWidth /. width)->Js.Math.floor->Float.toInt
  let framesBreak = editorContext.videoMeta.durationInFrames / maxFramesInScene

  Range.forEach(0, maxFramesInScene, i => {
    let svg = editorContext.wasmController.render_frame((i * framesBreak)->Js.BigInt.fromInt)
    let image = Image.make(~width, ~height=scene_height_size)

    image->Image.setSrc(svg->Image.btoa |> Js.String.concat("data:image/svg+xml;base64,"))
    image->Image.onLoad(() => {
      ctx->Canvas2d.save
      ctx->drawImage(
        ~imageData=image,
        ~dy=timeline_margin_y,
        ~dx=(timeline_margin_x /. 2. +. i->Js.Float.fromInt *. width)->Js.Math.floor,
        ~dirtyHeight=scene_height_size,
        ~dirtyWidth=width,
      )
      ctx->Canvas2d.restore
    })
  })

  ()
}

let renderScenesPlaceholder = (ctx, size, _editorContext: EditorContext.editorContext) => {
  ctx->clipOverTimeLineElement(~y=timeline_margin_y, ~width=size.maxSceneWidth)
  ctx->Canvas2d.setFillStyle(String, "#9ca3af")
  ctx->Canvas2d.fillRect(
    ~x=timeline_margin_x /. 2.0,
    ~y=timeline_margin_y,
    ~w=size.maxSceneWidth,
    ~h=scene_height_size,
  )
}

let renderAudioMap = (ctx, size, editorContext: EditorContext.editorContext) => {
  editorContext.videoMeta.audioMap->Option.forEach(audioMap =>
    audioMap->Js.Dict.keysToArray->Js.Array.reduce((startY, audioName) => {
      let (start_frame, end_frame) = audioMap->Js.Dict.get(audioName)->Utils.Option.unwrap

      let start_frame = start_frame->Js.Float.fromInt
      let end_frame = end_frame->Js.Float.fromInt

      let y = timeline_margin_y +. scene_height_size +. startY
      let x = timeline_margin_x /. 2.0 +. start_frame *. size.frameToPxRatio
      let width = (end_frame -. start_frame) *. size.frameToPxRatio

      ctx->Canvas2d.save
      ctx->Canvas2d.beginPath

      ctx->renderRoundedRect(~x, ~y, ~width, ~height=scene_height_size /. 2.0, ~radius=4.0, ())
      ctx->Canvas2d.clip

      ctx->Canvas2d.setFillStyle(String, "#059669")
      ctx->Canvas2d.fillRect(~x, ~y, ~w=width, ~h=scene_height_size /. 2.0)

      ctx->Canvas2d.setFillStyle(String, "#e2e8f0")
      audioName->Canvas2d.fillText(ctx, ~x=x +. 8., ~y=y +. 16.)

      ctx->Canvas2d.closePath
      ctx->Canvas2d.restore

      startY +. scene_height_size /. 2.0 +. 420.
    }, 32.)->ignore
  )
}

/// TODO MOVE

let renderTimeSlots = (ctx, size, editorContext: EditorContext.editorContext) => {
  let coordinate_step = 100
  let full_timestamp_each_steps = 2

  let stepsCount =
    size.maxSceneWidth
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

let renderSeekBar = (ctx, size, editorContext: EditorContext.editorContext) => {
  let x =
    (timeline_margin_x /. 2.0 +.
      editorContext.editorState.frame->Float.fromInt *. size.frameToPxRatio)->Js.Math.floor

  ctx->Canvas2d.beginPath
  ctx->Canvas2d.moveTo(~x, ~y=0.)
  ctx->Canvas2d.lineTo(~x, ~y=size.height)

  ctx->Canvas2d.moveTo(~x=x -. 2., ~y=0.)
  ctx->Canvas2d.lineTo(~x, ~y=7.)
  ctx->Canvas2d.lineTo(~x=x +. 2., ~y=0.)
  ctx->Canvas2d.lineTo(~x=x -. 2., ~y=0.)

  ctx->Canvas2d.setStrokeStyle(String, "#fbbf24")
  ctx->Canvas2d.stroke
  ctx->Canvas2d.closePath
}

@react.component
let make = (~sectionSize: UseEditorLayout.sectionSize) => {
  let canvasRef = React.useRef(Js.Nullable.null)
  let seekCanvasRef = React.useRef(Js.Nullable.null)
  let editorContext = EditorContext.useEditorContext()

  let canvasSize = React.useMemo4(() => {
    let scale = Web.Window.devicePixelRatio
    let maxSceneWidth = sectionSize.width -. timeline_margin_x

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

  React.useEffect1(() => {
    canvasRef.current
    ->Js.Nullable.toOption
    ->Belt.Option.map(element => {
      let ctx = Webapi.Canvas.CanvasElement.getContext2d(element)

      element->Canvas.CanvasElement.setHeight(canvasSize.scaledHeight->Js.Math.floor->Float.toInt)
      element->Canvas.CanvasElement.setWidth(canvasSize.scaledWidth->Js.Math.floor->Float.toInt)

      ctx->Canvas2d.scale(~x=canvasSize.scale, ~y=canvasSize.scale)
      ctx->renderTimeSlots(canvasSize, editorContext)

      switch editorContext.editorState.playState {
      | CantPlay => ctx->renderScenesPlaceholder(canvasSize, editorContext)
      | _ => {
          ctx->Canvas2d.save
          ctx->renderAudioMap(canvasSize, editorContext)
          ctx->Canvas2d.restore
          ctx->renderMainScene(canvasSize, editorContext)

          ()
        }
      }

      ()
    })
    ->ignore

    None
  }, [canvasSize])

  React.useEffect2(() => {
    seekCanvasRef.current
    ->Js.Nullable.toOption
    ->Belt.Option.map(seekCanvas => {
      let ctx = Webapi.Canvas.CanvasElement.getContext2d(seekCanvas)

      seekCanvas->Canvas.CanvasElement.setHeight(
        canvasSize.scaledHeight->Js.Math.floor->Float.toInt,
      )
      seekCanvas->Canvas.CanvasElement.setWidth(canvasSize.scaledWidth->Js.Math.floor->Float.toInt)

      ctx->Canvas2d.scale(~x=canvasSize.scale, ~y=canvasSize.scale)

      switch editorContext.editorState.playState {
      | CantPlay => ()
      | _ => renderSeekBar(ctx, canvasSize, editorContext)
      }->ignore
    })
    ->ignore

    None
  }, (canvasSize, editorContext.editorState.frame))

  <div className="relative">
    <canvas
      className="absolute inset-0"
      style={ReactDOMStyle.make(
        ~height=`${canvasSize.height->Float.toString}px`,
        ~width=`${canvasSize.width->Float.toString}px`,
        (),
      )}
      width={`${canvasSize.scaledWidth->Js.Math.floor->Float.toString}px`}
      height={`${canvasSize.scaledHeight->Js.Math.floor->Float.toString}px`}
      ref={ReactDOM.Ref.domRef(canvasRef)}
    />
    <canvas
      className="absolute inset-0"
      style={ReactDOMStyle.make(
        ~height=`${canvasSize.height->Float.toString}px`,
        ~width=`${canvasSize.width->Float.toString}px`,
        (),
      )}
      width={`${canvasSize.width->Js.Math.floor->Float.toString}px`}
      height={`${canvasSize.height->Js.Math.floor->Float.toString}px`}
      ref={ReactDOM.Ref.domRef(seekCanvasRef)}
    />
  </div>
}
