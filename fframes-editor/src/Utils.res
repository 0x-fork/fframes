module Array = {
  let last = arr => arr->Js.Array.get(arr->Array.length - 1)

  @send
  external spliceInPlace: (array<'a>, ~start: int, ~remove: int) => array<'a> = "splice"

  let removeInPlace = (arr, ~index) => spliceInPlace(arr, ~start=index, ~remove=1)
}

module Option = {
  let unwrap = option =>
    switch option {
    | Some(val) => val
    | None => failwith("expect option to contain value")
    }

  let unwrapOr = (option, ~default) =>
    switch option {
    | Some(val) => val
    | None => default
    }


  let some = (val) => Some(val)
}

module Log = {
  let andReturn = a => {
    Js.Console.log(a)
    a
  }
}

module Path = {
  let getFilename = path => {
    path->Js.String.replaceRegExp(%re("/^.*[\\\/]/"), "")
  }
}

module Float = {
  let divideWithReminder = (x, y) => {
    (Js.Math.floor(x /. y), Js.Float.mod(x, y))
  }
}

module Duration = {
  let leftPad = n =>
    n > 9.
      ? n->Js.Float.toFixedWithPrecision(~digits=0)
      : `0${n->Js.Float.toFixedWithPrecision(~digits=0)}`

  let formatSeconds = seconds => {
    let (hours, reminder) = Float.divideWithReminder(seconds, 3600.)
    let (minutes, seconds) = Float.divideWithReminder(reminder, 60.)

    if hours > 1.0 {
      `${hours->leftPad}:${minutes->leftPad}:${seconds->leftPad}`
    } else {
      `${minutes->leftPad}:${seconds->leftPad}`
    }
  }
}
