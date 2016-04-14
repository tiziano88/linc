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
  }


testModel : Model
testModel =
  { files =
    [ { name = "test.elm"
      , nextRef = 888
      , context =
        Context
        [ (0,
          { name = "num"
          , ref = 0
          , context = emptyContext
          , type_ = TInt
          , value = EInt 42
          })
        , (1,
          { name = "add"
          , ref = 1
          , context =
            Context
            [ (11,
              { name = "x"
              , ref = 11
              , context = emptyContext
              , type_ = TEmpty
              , value = EEmpty })
            , (12,
              { name = "y"
              , ref = 12
              , context = emptyContext
              , type_ = TEmpty
              , value = EEmpty })
            ]
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EApp
              { name = ""
              , ref = 111
              , context = emptyContext
              , type_ = TEmpty
              , value = ERef 11
              }
              { name = ""
              , ref = 112
              , context = emptyContext
              , type_ = TEmpty
              , value = ERef 100
              }
          })
        , (2,
          { name = "test"
          , ref = 2
          , context = emptyContext
          , type_ = TInt
          , value = EApp
              { name = ""
              , ref = 21
              , context = emptyContext
              , type_ = TEmpty
              , value = EApp
                { name = ""
                , ref = 211
                , context = emptyContext
                , type_ = TEmpty
                , value = ERef 1
                }
                { name = ""
                , ref = 212
                , context = emptyContext
                , type_ = TEmpty
                , value = ERef 0
                }
              }
              { name = ""
              , ref = 22
              , context = emptyContext
              , type_ = TEmpty
              , value = ERef 0
              }
          })
        , (3,
          { name = "error"
          , ref = 3
          , context = emptyContext
          , type_ = TInt
          , value = EApp
            { name = ""
            , ref = 31
            , context = emptyContext
            , type_ = TEmpty
            , value = ERef 111
            }
            { name = ""
            , ref = 32
            , context = emptyContext
            , type_ = TEmpty
            , value = ERef 0
            }
          })
        , (4,
          { name = "st"
          , ref = 4
          , context = emptyContext
          , type_ = TString
          , value = EString "test"
          })
        , (5,
          { name = "list"
          , ref = 5
          , context = emptyContext
          , type_ = TList TInt
          , value = EList
            [ { name = ""
              , ref = 51
              , context = emptyContext
              , type_ = TEmpty
              , value = ERef 0
              }
            , { name = ""
              , ref = 52
              , context = emptyContext
              , type_ = TEmpty
              , value = ERef 1
              }
            ]
          })
        , (6,
          { name = "cond"
          , ref = 6
          , context = emptyContext
          , type_ = TApp TBool TInt
          , value = EIf
            { name = ""
            , ref = 61
            , context = emptyContext
            , type_ = TEmpty
            , value = ERef 0
            }
            { name = ""
            , ref = 62
            , context = emptyContext
            , type_ = TEmpty
            , value = EInt 100
            }
            { name = ""
            , ref = 63
            , context = emptyContext
            , type_ = TEmpty
            , value = EInt 200
            }
          })
        ]
      }
    ]
  , currentRef = Just 0
  }


noEffects : a -> (a, Effects b)
noEffects m =
  (m, Effects.none)


type Action
  = Nop
  | SetCurrentRef ExprRef


type alias Address
  = Signal.Address Action


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    --AddObject o ->
      --noEffects { model | objects = Dict.insert model.nextObjRef o model.objects, nextObjRef = model.nextObjRef + 1 }

    Nop -> noEffects model

    SetCurrentRef ref -> noEffects { model | currentRef = Just ref }


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


htmlFile : Address -> Model -> File -> Html
htmlFile address model file =
  let xs = file.context
    |> mapContext
    |> List.map (htmlFunction address model)
  in Html.div [] xs


type alias Model =
  { files : List File
  , currentRef : Maybe ExprRef
  }


type Context = Context (List (WithRef Variable))


emptyContext : Context
emptyContext = Context []


mapContext : Context -> List Variable
mapContext (Context cs) =
  List.map snd cs


mergeContext : Context -> Context -> Context
mergeContext (Context cs1) (Context cs2) =
  Context (List.append cs1 cs2)


lookupContext : Context -> ExprRef -> Maybe Variable
lookupContext (Context cs) ref =
  cs
    |> List.filter (\(r, _) -> r == ref)
    |> List.head
    |> Maybe.map snd


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
  | EList (List Variable)
  | EString String
  | EIf Variable Variable Variable
  | EApp Variable Variable


type Symbol -- Unused.
  = SVar Variable
  | STyVar TypeVariable
  | STyCon TypeConstructor


type alias ExprRef = Int


type alias WithRef a = (ExprRef, a)


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


printExpr : Model -> Variable -> String
printExpr model v =
  case v.value of
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
            |> List.map (printExpr model)
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


htmlExpr : Address -> Model -> Variable -> Html
htmlExpr address model v =
  let content = case v.value of
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
      ([ Html.text "[" ] ++ (List.map (htmlExpr address model) ls) ++ [ Html.text "]" ])

    EIf cond eTrue eFalse ->
      [ Html.text "if"
      , htmlExpr address model cond
      , Html.text "then"
      , htmlExpr address model eTrue
      , Html.text "else"
      , htmlExpr address model eFalse
      ]

    EApp e1 e2 ->
      [ Html.text "("
      , htmlExpr address model e1
      , htmlExpr address model e2
      , Html.text ")"
      ]

  in
    Html.span
      [ onMouseEnter address (SetCurrentRef v.ref)
      , style <| [ "margin" => "5px"] ++
        (if
          (model.currentRef == Just v.ref)
        then
          [ "color" => "red" ]
        else
          [])
      ]
      content


printFunctionSignature : Model -> Variable -> String
printFunctionSignature model f =
  f.name ++ " : " ++ (printType f.type_)


(=>) : String -> String -> (String, String)
(=>) = (,)


htmlFunctionSignature : Address -> Model -> Variable -> Html
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
    , printExpr model f
    ]


htmlFunctionBody : Address -> Model -> Variable -> Html
htmlFunctionBody address model f =
  Html.div []
    [ Html.text f.name
    , f.context
      |> mapContext
      |> List.map printArg
      |> String.join " "
      |> Html.text
    , Html.text "="
    , htmlExpr address model f
    ]


printFunction : Model -> Variable -> String
printFunction model f =
  String.join "\n"
    [ (printFunctionSignature model f)
    , (printFunctionBody model f)
    ]

htmlFunction : Address -> Model -> Variable -> Html
htmlFunction address model f =
  Html.div []
    [ htmlFunctionSignature address model f
    , htmlFunctionBody address model f
    ]
