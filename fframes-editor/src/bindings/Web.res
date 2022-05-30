module Window = {
  @val @scope("window") external devicePixelRatio: float = "devicePixelRatio"
}

module Element = {
  @get external style: Webapi.Dom.Element.t => {..} = "style"

  let targetAsElement = %raw(`_ => _`)

  let isFocusable = el =>
    switch el->Webapi.Dom.Element.tagName {
    | "TEXTAREA" | "SELECT" | "INPUT" | "BUTTON" | "A" => true
    | _ =>
      switch el |> Webapi.Dom.Element.getAttribute("role") {
      | Some("slider") | Some("input") | Some("button") | Some("checkbox") | Some("link") => true
      | _ => false
      }
    }
}

module Float32Array = {
  @get external length: Js.Float32Array.t => float = "length"
  @get_index external get: (Js.Float32Array.t, int) => float = ""
}
