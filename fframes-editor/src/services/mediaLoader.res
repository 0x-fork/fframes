type defaultModule<'a> = {default: 'a}
type mediaImport<'a> = unit => Js.Promise.t<'a>

type mediaImportHash<'a> = Js.Dict.t<mediaImport<'a>>

// let loadAudio = (url) => { 

// }

let makeMediaEditor = (imports, name) => {
  let a =
    imports
    ->Js.Dict.get(name)
    ->Belt.Option.map(val =>
      switch name {
      | name if name->Js.String.endsWith(".mp3") => 0
      }
    )
}
