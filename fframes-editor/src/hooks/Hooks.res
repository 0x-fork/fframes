include UseDimensions
include UseEditorLayout

let useEvent = (fn: 'a => unit) => {
  let ref = React.useRef(fn)

  React.useLayoutEffect(() => {
    ref.current = fn
    None
  })

  React.useCallback0(arg => ref.current(arg))
}
