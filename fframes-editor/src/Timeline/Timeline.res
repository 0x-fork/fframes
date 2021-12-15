module Canvas = Webapi.Canvas
module Canvas2d = Webapi.Canvas.Canvas2d

let renderCanvas = (element, context) => {
  let width = float_of_int(Canvas.CanvasElement.width(element))
  let height = float_of_int(Canvas.CanvasElement.height(element))
  let centerX = width /. 2.0
  let centerY = height /. 2.0

  context->Canvas2d.setFillStyle(String, "white")
  context->Canvas2d.fillRect(~x=centerX, ~y=0.0, ~w=width /. 2.0, ~h=height /. 2.0)
}

@react.component
let make = () => {
  let canvasRef = React.useRef(Js.Nullable.null)

  React.useEffect0(() => {
    // %debugger
    canvasRef.current
    ->Js.Nullable.toOption
    ->Belt.Option.forEach(element => {
      let context = Webapi.Canvas.CanvasElement.getContext2d(element)
      renderCanvas(element, context)
    })

    None
  })

  <canvas height="400px" width="calc(100vw)" ref={ReactDOM.Ref.domRef(canvasRef)} />
}
