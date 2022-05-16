open Belt
open CanvasSize

module Canvas = Webapi.Canvas
module Canvas2d = Webapi.Canvas.Canvas2d

let renderSeekBar = (ctx, size, playState: Player.state) => {
  let x =
    (Float.fromInt(timeline_margin_x / 2) +. playState.frame->Float.fromInt *. size.frameToPxRatio)
      ->Js.Math.floor

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

let calculateFrameFromEvent = (e, ~size) => {
  let rectLeft =
    e
    ->ReactEvent.Mouse.target
    ->Web.Element.targetAsElement
    ->Webapi.Dom.Element.getBoundingClientRect
    ->Webapi.Dom.DomRect.left
    ->Int.fromFloat

  let x = e->ReactEvent.Mouse.clientX - rectLeft - timeline_margin_x / 2

  if x > 0 {
    (x->Float.fromInt *. size.pxToFrameRation)->Int.fromFloat
  } else {
    0
  }
}

@react.component
let make = (~size) => {
  let seekCanvasRef = React.useRef(Js.Nullable.null)
  let editorContext = EditorContext.useEditorContext()
  let (player, dispatch) = editorContext.usePlayer()

  React.useEffect3(() => {
    seekCanvasRef.current
    ->Js.Nullable.toOption
    ->Belt.Option.map(canvasElement => {
      let ctx = Webapi.Canvas.CanvasElement.getContext2d(canvasElement)

      canvasElement->Canvas.CanvasElement.setHeight(size.scaledHeight->Js.Math.floor->Float.toInt)
      canvasElement->Canvas.CanvasElement.setWidth(size.scaledWidth->Js.Math.floor->Float.toInt)

      ctx->Canvas2d.scale(~x=size.scale, ~y=size.scale)

      switch player.playState {
      | CantPlay => ()
      | _ => renderSeekBar(ctx, size, player)
      }->ignore
    })
    ->ignore

    None
  }, (size, player.frame, player.playState))

  let hanldeMouseMove = e => {
    if player.playState !== Playing {
      dispatch(NewFrame(calculateFrameFromEvent(e, ~size)))
    }
  }

  let handleClick = e => {
    let frame = calculateFrameFromEvent(e, ~size)

    dispatch(NewFrame(frame))
    dispatch(Play)
  }

  <div className="relative">
    <canvas
      onClick=handleClick
      onMouseMove=hanldeMouseMove
      className="absolute inset-0"
      style={ReactDOMStyle.make(
        ~height=`${size.height->Float.toString}px`,
        ~width=`${size.width->Float.toString}px`,
        (),
      )}
      width={`${size.width->Js.Math.floor->Float.toString}px`}
      height={`${size.height->Js.Math.floor->Float.toString}px`}
      ref={ReactDOM.Ref.domRef(seekCanvasRef)}
    />
  </div>
}
