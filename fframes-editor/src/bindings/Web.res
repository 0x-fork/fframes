module Window = {
  @val @scope("window") external devicePixelRatio: float = "devicePixelRatio"
}

module Element = {
  @get external style: Webapi.Dom.Element.t => {..} = "style"

  let targetAsElement =  %raw(`_ => _`)
}

module Float32Array = {
  @get external length: Js.Float32Array.t => float = "length"
  @get_index external get: (Js.Float32Array.t, int) => float = ""
}
