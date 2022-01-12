open Belt
open Cx

module LoadedMediaIcon = {
  let iconClassName = "overflow-hidden bg-gray-400 h-10 w-10 2xl:h-12 2xl:w-12 rounded-xl bg-gradient-to-r from-indigo-400 to-pink-400 flex justify-center items-center"

  @react.component
  let make = (~media: MediaLoader.processedMedia) => {
    switch media {
    | Image({src}) =>
      <div
        className={cx(["bg-cover bg-no-repeat bg-center", iconClassName])}
        style={ReactDOMStyle.make(~backgroundImage=`url(${src})`, ())}
      />

    | nonImageMedia =>
      <div className=iconClassName>
        {switch nonImageMedia {
        | Audio(_) => <Icons.MusicalNotesIcon color="currentColor" className="h-7 w-7" />
        | Font(_) => <Icons.FontIcon color="currentColor" className="h-7 w-7" />
        | Subtitles(_) => <Icons.CaptionsIcon color="currentColor" className="h-7 w-7" />
        | _ => React.null
        }}
      </div>
    }
  }
}

module LoadedMedia = {
  @react.component
  let make = (~name, ~media: MediaLoader.processedMedia) => {
    <div className="flex  space-x-2">
      <LoadedMediaIcon media />
      <div className="flex flex-col">
        <p className="text-gray-300 2xl:text-lg"> {name->React.string} </p>
        <p className="text-gray-500 text-xs 2xl:text-base">
          {switch media {
          | Audio({sampleRate, duration}) =>
            `${duration->Utils.Duration.formatSeconds}, ${sampleRate->Int.toString}hz`->React.string
          | Font(fontInfo) => React.string(fontInfo)
          | Subtitles => React.string("s")
          | Image({width, height}) => `${width->Int.toString}x${height->Int.toString}`->React.string
          | _ => React.null
          }}
        </p>
      </div>
    </div>
  }
}

module Loading = {
  @react.component
  let make = (~name) => {
    <div> <p> {name->React.string} </p> </div>
  }
}

@react.component
let make = () => {
  let mediaState = MediaLoader.MediaLoaderObserver.useObservable()

  <ul className="divide-y divide-gray-800">
    {mediaState.mediaList
    ->Map.String.keysToArray
    ->Array.map(name => {
      let media = mediaState.mediaList->Map.String.getExn(name)

      <li className="px-4 py-2 h-16 2xl:h-20 flex flex-col justify-center">
        {switch media {
        | Media(media) => <LoadedMedia name media />
        | Loading(_) => <Loading name />
        | _ => React.null
        }}
      </li>
    })
    ->React.array}
  </ul>
}
