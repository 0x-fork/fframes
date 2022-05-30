module RadixSlider = {
  module Root = {
    @react.component @module("@radix-ui/react-slider")
    external make: (
      ~value: array<float>,
      ~onValueChange: array<float> => unit=?,
      ~step: float,
      ~min: float,
      ~max: float,
      ~children: React.element,
      ~className: string=?,
    ) => React.element = "Root"
  }

  module Track = {
    @react.component @module("@radix-ui/react-slider")
    external make: (~children: React.element, ~className: string=?) => React.element = "Track"
  }

  module Range = {
    @react.component @module("@radix-ui/react-slider")
    external make: (~className: string=?) => React.element = "Range"
  }

  module Thumb = {
    @react.component @module("@radix-ui/react-slider")
    external make: (~className: string=?) => React.element = "Thumb"
  }
}

@react.component
let make = (~onValueChange, ~value, ~min, ~max, ~step) => {
  let handleChange = React.useCallback1(newValue => {
    newValue[0]->onValueChange
  }, [onValueChange])
  
  <RadixSlider.Root
    step
    min
    max
    value=[value]
    onValueChange=handleChange
    className="relative flex items-center select-none w-28 h-4 mx-2">
    <RadixSlider.Track className="relative flex-grow h-1 rounded-full bg-slate-800">
      <RadixSlider.Range className="absolute bg-gray-100 rounded-full h-full" />
    </RadixSlider.Track>
    <RadixSlider.Thumb
      className="block cursor-grab w-[13px] h-[13px] bg-white transition-transform shadow-xl rounded-full focus:bg-gradient-to-tr from-indigo-400 to-pink-400"
    />
  </RadixSlider.Root>
}
