module Window = {
  @val @scope("window") external devicePixelRatio: float = "devicePixelRatio"
}

module Element = {
  @get external style: Webapi.Dom.Element.t => {..} = "style"
}
