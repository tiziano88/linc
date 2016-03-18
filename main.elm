import Array
import Dict
import Effects exposing (Effects)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import StartApp
import String
import Task
import Time


main : Signal Html
main =
  app.html


port tasks : Signal (Task.Task Effects.Never ())
port tasks =
  app.tasks


app =
  StartApp.start
    { init = init
    , view = view
    , update = update
    , inputs = []
    }


init : (Model, Effects Action)
init =
  noEffects testModel


initialModel : Model
initialModel =
  { defs = empty
  }


testModel : Model
testModel =
  { defs =
    empty
    |> insert
      { name = "num"
      , args = empty
      , type_ = TInt
      , value = EInt 42
      }
    |> insert
      {name = "add"
      , args =
        empty
        |> insert {name = "x", type_ = TInt}
        |> insert {name = "y", type_ = TInt}
      , type_ = TInt
      , value = EInt 100
      }
  }


noEffects : a -> (a, Effects b)
noEffects m =
  (m, Effects.none)


type Action
  = Nop
  --| AddObject Object


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    --AddObject o ->
      --noEffects { model | objects = Dict.insert model.nextObjRef o model.objects, nextObjRef = model.nextObjRef + 1 }

    Nop -> noEffects model


view address model =
  Html.div []
    [ Html.button
      --[ onClick address <| AddObject { name = "test" } ]
      []
      [ Html.text "Add Object" ]
    , Html.div [] [ Html.text <| toString model ]
    , Html.pre [] [ Html.text <| String.join "\n" <| List.map printFunction <| Dict.values model.defs.things ]
    ]


type alias Model =
  { defs : Bag FunctionRef Function
  }


type alias Object =
  { name : String
  }


type Type
  = TInt
  | TApp Type Type


type Expr
  = EInt Int
  | EApp FunctionRef Expr


type alias ArgRef = Int

type alias Arg =
  { name : String
  , type_ : Type
  }


type alias Bag number v =
  { things : Dict.Dict number v
  , nextRef : number
  }


-- Maybe just use global (per-file) nextRef?
empty : Bag number v
empty =
  { things = Dict.empty
  , nextRef = 0
  }


insert : v -> Bag number v -> Bag number v
insert v bag =
  { things = Dict.insert bag.nextRef v bag.things
  , nextRef = bag.nextRef + 1
  }


--get : number -> Bag number v -> Maybe v
--get ref bag =
  --Dict.get ref (bag.things)


type alias FunctionRef = Int


type alias Function =
  { name : String
  , type_ : Type
  , args : Bag ArgRef Arg
  , value : Expr
  }


printArg : Arg -> String
printArg a =
  a.name ++ " :: " ++ (toString a.type_)

printFunction : Function -> String
printFunction f =
  f.name ++ " :: {" ++ (f.args.things |> Dict.values |> List.map printArg |> String.join ", ") ++ "} -> " ++ (toString f.type_)
