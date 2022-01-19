module type AnimationRuntime = {
  let a: float
}

module RafRuntime: AnimationRuntime = {
  let a = Js.Math.random()
}

module AudioContextRuntime: AnimationRuntime = {
  let a = Js.Math.random()
}

let getModule = (a): module(AnimationRuntime) => {
  switch a > 15 {
  | true => module(RafRuntime)
  | false => module(AudioContextRuntime)
  }
}
