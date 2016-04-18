import Array
import Dict
import Effects exposing (Effects)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Signal
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
  , currentRef = Nothing
  , currentExpr = EEmpty
  }


testModel : Model
testModel =
  { files =
    [ { name = "test.elm"
      , nextRef = 888
      , context =
        Context <| Array.fromList
        [ { name = "num"
          , ref = 0
          , context = emptyContext
          , type_ = TInt
          , value = EInt 42
          }
        , { name = "add"
          , ref = 1
          , context =
            Context <| Array.fromList
            [ { name = "x"
              , ref = 11
              , context = emptyContext
              , type_ = TEmpty
              , value = EEmpty }
            , { name = "y"
              , ref = 12
              , context = emptyContext
              , type_ = TEmpty
              , value = EEmpty }
            ]
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EApp (ERef 11) (ERef 100)
          }
        , { name = "test"
          , ref = 2
          , context = emptyContext
          , type_ = TInt
          , value = EApp (EApp (ERef 1) (ERef 0)) (ERef 0)
          }
        , { name = "error"
          , ref = 3
          , context = emptyContext
          , type_ = TInt
          , value = EApp (ERef 111) (ERef 0)
          }
        , { name = "st"
          , ref = 4
          , context = emptyContext
          , type_ = TString
          , value = EString "test"
          }
        , { name = "list"
          , ref = 5
          , context = emptyContext
          , type_ = TList TInt
          , value = EList (Array.fromList [(ERef 0), (ERef 1)])
          }
        , { name = "cond"
          , ref = 6
          , context = emptyContext
          , type_ = TApp TBool TInt
          , value = EIf (ERef 0) (EInt 100) (EInt 200)
          }
        ]
      }
    ]
  , currentRef = Just 0
  , currentExpr = EEmpty
  }


noEffects : a -> (a, Effects b)
noEffects m =
  (m, Effects.none)


type Action
  = Nop
  | SetCurrentRef ExprRef
  | SetExpr ExprRef Expr


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    --AddObject o ->
      --noEffects { model | objects = Dict.insert model.nextObjRef o model.objects, nextObjRef = model.nextObjRef + 1 }

    Nop -> noEffects model

    SetCurrentRef ref -> noEffects { model | currentRef = Just ref }

    SetExpr ref e -> noEffects
      { model
      | files =
        List.map (\f -> { f | context = updateContext f.context ref e }) model.files
      , currentRef = Just ref
      , currentExpr = e
      }


view address model =
  Html.div []
    [ Html.button
      --[ onClick address <| AddObject { name = "test" } ]
      []
      [ Html.text "Add Object" ]
    , Html.div [] [ Html.text <| toString model ]
    , Html.pre [] (model.files |> List.map (htmlFile address model))
    ]


printFile : Model -> File -> String
printFile model file =
  file.context
    |> mapContext
    |> List.map (printFunction model)
    |> String.join "\n\n\n"


htmlFile : Signal.Address Action -> Model -> File -> Html
htmlFile address model file =
  let xs = file.context
    |> mapContextIndexed
    |> List.map (\(i, e) -> (htmlFunction address (Signal.forwardTo address <| SetExpr e.ref) model e))
  in Html.div [] xs


type alias Model =
  { files : List File
  , currentRef : Maybe ExprRef
  , currentExpr : Expr
  }


type Context = Context (Array.Array Variable)


emptyContext : Context
emptyContext = Context Array.empty


mapContextIndexed : Context -> List (Int, Variable)
mapContextIndexed (Context cs) =
  Array.toIndexedList cs


mapContext : Context -> List Variable
mapContext (Context cs) =
  Array.toList cs


mergeContext : Context -> Context -> Context
mergeContext (Context cs1) (Context cs2) =
  Context (Array.append cs1 cs2)


lookupContext : Context -> ExprRef -> Maybe Variable
lookupContext (Context cs) ref =
  cs
    |> Array.filter (\v -> v.ref == ref)
    |> Array.get 0


updateContext : Context -> ExprRef -> Expr -> Context
updateContext (Context cs) ref e =
  cs
    |> Array.map (\v -> if v.ref == ref then {v | value = e } else v)
    |> Context


type alias File =
  { name : String
  , nextRef : ExprRef
  , context : Context
  }


type alias Variable =
  { name : String
  , ref : ExprRef
  , type_ : Type
  , context : Context
  , value : Expr
  }

type alias Definition =
  { variable : Variable
  , value : Expr
  }


type alias Node =
  { ref : ExprRef
  , value : Expr
  }


type alias TypeVariable =
  { name : String
  , kind : String -- ?
  }


type alias TypeConstructor =
  { name : String
  }


type Type
  = TEmpty -- Args.
  | TInt
  | TBool
  | TString
  | TList Type
  | TApp Type Type


type Expr
  = EEmpty -- Args.
  | ERef ExprRef
  | EInt Int
  | EBool Bool
  | EList (Array.Array Expr)
  | EString String
  | EIf Expr Expr Expr
  | EApp Expr Expr


type Symbol -- Unused.
  = SVar Variable
  | STyVar TypeVariable
  | STyCon TypeConstructor


type alias ExprRef = Int


getVariable : Model -> ExprRef -> Maybe Variable
getVariable model ref =
  model.files
    |> List.map (\x -> getFileFunctionRef x ref)
    |> Maybe.oneOf


getFileFunctionRef : File -> ExprRef -> Maybe Variable
getFileFunctionRef file ref =
  let
    c1 = file.context
    c2 =
      file.context
        |> mapContext
        |> List.map (\x -> x.context)
        |> List.foldl mergeContext emptyContext
    c = mergeContext c1 c2
  in
    lookupContext c ref


printArg : Variable -> String
printArg a =
  a.name


printType : Type -> String
printType t =
  case t of
    TEmpty -> "<<<EMPTY>>>"
    TInt -> "Int"
    TBool -> "Bool"
    TString -> "String"
    TList t -> "List " ++ (printType t)
    TApp t1 t2 -> "(" ++ (printType t1) ++ " -> " ++ (printType t2) ++ ")"


printExpr : Model -> Expr -> String
printExpr model e =
  case e of
    EEmpty ->
      "<<<EMPTY>>>"

    ERef r ->
      let
        mf = getVariable model r
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
        s =
          ls
            |> Array.map (printExpr model)
            |> Array.toList
            |> String.join ", "
      in
        "[" ++ s ++ "]"

    EIf cond eTrue eFalse ->
      String.join " "
        [ "if"
        , printExpr model cond
        , "then"
        , printExpr model eTrue
        , "else"
        , printExpr model eFalse
        ]

    EApp e1 e2 ->
      String.join " "
        [ "(" ++ printExpr model e1
        , printExpr model e2 ++ ")"
        ]


htmlExpr : Signal.Address Action -> Signal.Address Expr -> Model -> Expr -> Html
htmlExpr address aexpr model e =
  let
    content = case e of
      EEmpty ->
        [ Html.text "<<<EMPTY>>>" ]

      ERef r ->
        let
          mf = getVariable model r
        in
         case mf of
           Just f -> [ Html.text f.name ]
           Nothing -> [ Html.text "<<<ERROR>>>" ]

      EInt v ->
        [ Html.text <| toString v ]

      EBool v ->
        [ Html.text <| toString v ]

      EString v ->
        [ Html.text <| "\"" ++ v ++ "\"" ]

      EList ls ->
        ([ Html.text "[" ] ++ (Array.map (htmlExpr address aexpr model) ls |> Array.toList) ++ [ Html.text "]" ])

      EIf cond eTrue eFalse ->
        [ Html.text "if"
        , htmlExpr address (Signal.forwardTo aexpr (\x -> EIf x eTrue eFalse)) model cond
        , Html.text "then"
        , htmlExpr address (Signal.forwardTo aexpr (\x -> EIf cond x eFalse)) model eTrue
        , Html.text "else"
        , htmlExpr address (Signal.forwardTo aexpr (\x -> EIf cond eTrue x)) model eFalse
        ]

      EApp e1 e2 ->
        [ Html.text "("
        , htmlExpr address (Signal.forwardTo aexpr (\x -> EApp x e2)) model e1
        , htmlExpr address (Signal.forwardTo aexpr (\x -> EApp e1 x)) model e2
        , Html.text ")"
        ]

    ref = (case e of
      ERef r -> (model.currentRef == Just r)
      _ -> False)

  in
    Html.span
      [ style <| [ "margin" => "5px"] ++
        (if
          ref
        then
          [ "color" => "blue" ]
        else
          [])
      ]
      (content ++
      [ Html.a
        [ onClick address Nop ]
        [ Html.text " [nop] " ]
      , Html.a
        [ onClick aexpr <| EIf EEmpty EEmpty EEmpty ]
        [ Html.text " [if] " ]
      , Html.a
        [ onClick aexpr EEmpty ]
        [ Html.text " [x] " ]
      ])


printFunctionSignature : Model -> Variable -> String
printFunctionSignature model f =
  f.name ++ " : " ++ (printType f.type_)


(=>) : String -> String -> (String, String)
(=>) = (,)


htmlFunctionSignature : Signal.Address Action -> Model -> Variable -> Html
htmlFunctionSignature address model f =
  Html.div []
    [ Html.text f.name
    , Html.text " : "
    , Html.text <| (printType f.type_)
    ]



printFunctionBody : Model -> Variable -> String
printFunctionBody model f =
  String.join " "
    [ f.name
    , f.context
      |> mapContext
      |> List.map printArg
      |> String.join " "
    , "="
    , printExpr model f.value
    ]


htmlFunctionBody : Signal.Address Action -> Signal.Address Expr -> Model -> Variable -> Html
htmlFunctionBody address aexpr model f =
  Html.div []
    [ Html.text f.name
    , f.context
      |> mapContext
      |> List.map printArg
      |> String.join " "
      |> Html.text
    , Html.text "="
    , htmlExpr address aexpr model f.value
    ]


printFunction : Model -> Variable -> String
printFunction model f =
  String.join "\n"
    [ (printFunctionSignature model f)
    , (printFunctionBody model f)
    ]

htmlFunction : Signal.Address Action -> Signal.Address Expr -> Model -> Variable -> Html
htmlFunction address aexpr model v =
  Html.div []
    [ htmlFunctionSignature address model v
    , htmlFunctionBody address aexpr model v
    ]
