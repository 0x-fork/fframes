type t

@new external make: (~width: float, ~height: float) => t = "Image"

@set
external onLoad: (t, unit => unit) => unit = "onload"

@set
external setSrc: (t, string) => unit = "src"

@scope("window") @val
external btoa: string => string = "btoa"
