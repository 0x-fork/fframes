module MusicalNotesIcon = {
  @react.component @module("./MusicalNoteIcon")
  external make: (~color: string=?, ~className: string=?) => React.element = "MusicalNoteIcon"
}
module FontIcon = {
  @react.component @module("./FontIcon")
  external make: (~color: string=?, ~className: string=?) => React.element = "FontIcon"
}
module CaptionsIcon = {
  @react.component @module("./CaptionsIcon")
  external make: (~color: string=?, ~className: string=?) => React.element = "CaptionsIcon"
}
