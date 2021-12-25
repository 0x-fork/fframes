open Webapi

module DocumentEvent = Dom.EventTarget.Impl(Dom.Window)

type dimensions = {
  width: int,
  height: int,
}

let getDimensions = _ => {
  width: Dom.window->Dom.Window.innerWidth,
  height: Dom.window->Dom.Window.innerHeight,
}

let useDimensions = () => {
  let (dimensions, setDimesions) = React.useState(getDimensions)

  React.useLayoutEffect0(() => {
    let handleResize = _ => {
      setDimesions(getDimensions)
    }

    Dom.window
    |> DocumentEvent.asEventTarget
    |> Dom.EventTarget.addEventListener("resize", handleResize)

    Some(
      () =>
        Dom.window
        |> DocumentEvent.asEventTarget
        |> Dom.EventTarget.removeEventListener("resize", handleResize),
    )
  })

  dimensions
}
