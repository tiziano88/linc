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
  { files = []
  }


testModel : Model
testModel =
  { files =
    [ { name = "test.elm"
      , nextRef = 888
      , defs =
        [ (0,
          { name = "num"
          , args = []
          , type_ = TInt
          , value = EInt 42
          })
        , (1,
          { name = "add"
          , args =
            [ (11, { name = "x" })
            , (12, { name = "y" })
            ]
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EApp (ERef 11) (EInt 100)
          })
        , (2,
          { name = "test"
          , args = []
          , type_ = TInt
          , value = EApp (EApp (ERef 1) (ERef 0)) (ERef 0)
          })
        , (3,
          { name = "error"
          , args = []
          , type_ = TInt
          , value = EApp (ERef 111) (ERef 0)
          })
        , (4,
          { name = "st"
          , args = []
          , type_ = TString
          , value = EString "test"
          })
        , (5,
          { name = "list"
          , args = []
          , type_ = TList TInt
          , value = EList [(ERef 0), (ERef 1)]
          })
        , (6,
          { name = "cond"
          , args = []
          , type_ = TApp TBool TInt
          , value = EIf (ERef 0) (EInt 100) (EInt 200)
          })
        ]
      }
    ]
  }


noEffects : a -> (a, Effects b)
noEffects m =
  (m, Effects.none)


type Action
  = Nop


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
    , Html.pre [] (model.files |> List.map (printFile model)|> List.map Html.text)
    ]


printFile : Model -> File -> String
printFile model file =
  file.defs |> List.map snd |> List.map (printFunction model) |> String.join "\n\n\n"


type alias Model =
  { files : List File
  }


type alias File =
  { name : String
  , nextRef : ExprRef
  , defs : List (WithRef Function)
  }


type Type
  = TInt
  | TBool
  | TString
  | TList Type
  | TApp Type Type


type Expr
  = ERef ExprRef
  | EInt Int
  | EBool Bool
  | EList (List Expr)
  | EString String
  | EIf Expr Expr Expr
  | EApp Expr Expr


type Def
  = DFun Function
  | DArg Arg


type alias ExprRef = Int


type alias WithRef a = (ExprRef, a)


getFunctionRef : Model -> ExprRef -> Maybe Function
getFunctionRef model ref =
  model.files |> List.map (\x -> getFileFunctionRef x ref) |> Maybe.oneOf


getFileFunctionRef : File -> ExprRef -> Maybe Function
getFileFunctionRef file ref =
  let
    r1 = file.defs |> List.filter (\(r, _) -> r == ref) |> List.map snd |> List.head
  in
    r1


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


type alias Function =
  { name : String
  , type_ : Type
  , args : List (WithRef Arg)
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
    TString -> "String"
    TList t -> "List " ++ (printType t)
    TApp t1 t2 -> "(" ++ (printType t1) ++ " -> " ++ (printType t2) ++ ")"


printExpr : Model -> Expr -> String
printExpr model e =
  case e of
    ERef r ->
      let
        mf = getFunctionRef model r
      in
       case mf of
         Just f -> f.name
         Nothing -> "<<<ERROR>>>"

    EInt v ->
      toString v

    EBool v ->
      toString v

    EString v ->
      "\"" ++ v ++ "\""

    EList ls ->
      let
        s = ls |> List.map (printExpr model) |> String.join ", "
      in
        "[" ++ s ++ "]"

    EIf cond eTrue eFalse ->
      String.join " " <| ["if", printExpr model cond, "then", printExpr model eTrue, "else", printExpr model eFalse]


    EApp e1 e2 ->
      "(" ++ (printExpr model e1) ++ " " ++ (printExpr model e2) ++ ")"


printFunction : Model -> Function -> String
printFunction model f =
  f.name ++ " : " ++ (printType f.type_)
  ++ "\n" ++ f.name ++ " " ++ (f.args |> List.map snd |> List.map printArg |> String.join " ") ++ " = " ++ (printExpr model f.value)
