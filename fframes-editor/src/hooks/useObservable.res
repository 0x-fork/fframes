module type Observable = {
  type state
  type action

  let initial: state

  let reducer: (state, action) => state
}

type unlisten = unit => unit

module MakeObserver = (Observable: Observable) => {
  type t = Observable.state
  type action = Observable.action

  type listener = Observable.state => unit
  type listenerId = {id: int, listener: listener}

  let mutableState = ref(Observable.initial)
  let listeners: array<listenerId> = []

  let get = () => mutableState.contents
  let dispatch = action => {
    mutableState := Observable.reducer(mutableState.contents, action)
  }

  let nextId = ref(0)
  let subscribe = listener => {
    let id = nextId.contents
    nextId := id + 1
    listeners->Js.Array.push({id: id, listener: listener})->ignore

    () => {
      let index = listeners->Js.Array.findIndex(listener => listener.id === id)

      if index >= 0 {
        listeners->Utils.Array.removeInPlace(~index)->ignore
      }
    }
  }

  let useObservable = () => {
    let (_, forceUpdate) = React.useReducer((x, _) => x + 1, 0)

    React.useEffect0(() => {
      Some(subscribe(forceUpdate))
    })

    get()
  }
}