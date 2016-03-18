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
        |> insert {name = "x"}
        |> insert {name = "y"}
      , type_ = TApp TInt (TApp TInt TInt)
      , value = EInt 100
      }
    |> insert
      { name = "test"
      , args = empty
      , type_ = TInt
      , value = EApp (EApp (ERef 1) (ERef 0)) (ERef 0)
      }
    |> insert
      { name = "error"
      , args = empty
      , type_ = TInt
      , value = EApp (ERef 111) (ERef 0)
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
    , Html.pre [] [ Html.text <| String.join "\n" <| List.map (printFunction model) <| Dict.values model.defs.things ]
    ]


type alias Model =
  { defs : Bag FunctionRef Function
  }


type alias Object =
  { name : String
  }


type Type
  = TInt
  | TBool
  | TApp Type Type


type Expr
  = ERef ExprRef
  | EInt Int
  | EBool Bool
  | EApp Expr Expr

type alias ExprRef = Int

type alias ArgRef = Int

type alias Arg =
  { name : String
  }


type alias Bag number v =
  { things : Dict.Dict number v
  , nextRef : number
  }


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
  a.name


printType : Type -> String
printType t =
  case t of
    TInt -> "Int"
    TBool -> "Bool"
    TApp t1 t2 -> "(" ++ (printType t1) ++ " -> " ++ (printType t2) ++ ")"


printExpr : Model -> Expr -> String
printExpr model e =
  case e of
    ERef r ->
      Dict.get r model.defs.things
      |> Maybe.map (\x -> x.name)
      |> Maybe.withDefault "<<<ERROR>>>"

    EInt v ->
      toString v

    EBool v ->
      toString v

    EApp e1 e2 ->
      "(" ++ (printExpr model e1) ++ " " ++ (printExpr model e2) ++ ")"


printFunction : Model -> Function -> String
printFunction model f =
  f.name ++ " : " ++ (printType f.type_)
  ++ "\n" ++ f.name ++ " " ++ (f.args.things |> Dict.values |> List.map printArg |> String.join " ") ++ " = " ++ (printExpr model f.value)
