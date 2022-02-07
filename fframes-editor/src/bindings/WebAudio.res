module AudioParam = {
  type t

  @get external getDefaultValue: t => float = "defaultValue"

  @get external getMaxValue: t => float = "maxValue"

  @get external getMinValue: t => float = "minValue"

  @get external getValue: t => float = "value"

  @set external setValue: (t, float) => unit = "value"

  @send external setValueAtTime: (t, ~value: float, ~startTime: float) => unit = "setValueAtTime"

  @send
  external linearRampToValueAtTime: (t, ~value: float, ~endTime: float) => t =
    "linearRampToValueAtTime"

  @send
  external exponentialRampToValueAtTime: (t, ~value: float, ~endTime: float) => t =
    "exponentialRampToValueAtTime"

  @send
  external setTargetAtTime: (t, ~target: float, ~startTime: float, ~timeConstant: float) => t =
    "setTargetAtTime"

  @send
  external setValueCurveAtTime: (
    t,
    ~values: array<float>,
    ~startTime: float,
    ~duration: float,
  ) => t = "SetValueCurveAtTime"

  @send external cancelScheduledValues: (t, ~startTime: float) => t = "cancelScheduledValues"

  @send external cancelAndHoldAtTime: (t, ~cancelTime: float) => t = "cancelAndHoldAtTime"
}

module AudioBuffer = {
  type t
}

module Node = {
  type t = {"gain": {@set "value": float}}

  @send external connect: (t, t) => unit = "connect"
  @send external disconnect: t => unit = "disconnect"
  @get external getNumberOfInputs: t => int = "numberOfInputs"

  @get external getGain: t => AudioParam.t = "gain"
  @set external setGainLevel: (t, float) => unit = "gain.value"
  @send external start: (t, float) => unit = "start"

  @set
  external setBuffer: (t, AudioBuffer.t) => unit = "buffer"

  let setGainValue = (gainNode, ~value, ~startTime) =>
    gainNode->getGain->AudioParam.setValueAtTime(~value, ~startTime)
}

module Context = {
  type t = { 
    destination: Node.t
  }

  @new external create: unit => t = "AudioContext"
  @get external getDestination: t => Node.t = "destination"
  @get external getCurrentTime: t => float = "currentTime"
  @send external createGain: t => Node.t = "createGain"
  @send external createOscillator: t => Node.t = "create"
  @send
  external createMediaElementSource: (t, Dom.element) => Node.t = "createMediaElementSource"
  @send
  external createBufferSource: t => Node.t = "createBufferSource"

  // AudioNode
  @get external fromAudioNode: Node.t => t = "context"
  @send
  external decodeAudioData: Js.ArrayBuffer.t => Js.Promise.t<AudioBuffer.t> = "decodeAudioData"
}
