import Array
import Dict
import Html.App as Html
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import String
import Task
import Time


main : Program Never
main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = always Sub.none
    }


init : (Model, Cmd Msg)
init =
  noEffects testModel


initialModel : Model
initialModel =
  { files = []
  , parent = Dict.empty
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
            []
            --[ { name = "x"
              --, ref = 11
              --, context = emptyContext
              --, type_ = TEmpty
              --, value = EEmpty }
            --, { name = "y"
              --, ref = 12
              --, context = emptyContext
              --, type_ = TEmpty
              --, value = EEmpty }
            --]
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
          , value = EIf (ERef 0) (EInt 100) (ESel (EInt 200))
          }
        ]
      }
    ]
  , parent = Dict.empty
  , currentRef = Just 0
  , currentExpr = EEmpty
  }


noEffects : a -> (a, Cmd b)
noEffects m =
  (m, Cmd.none)


type Msg
  = Nop
  | SetCurrentRef ExprRef
  | SetExpr ExprRef Expr
  | ClearSel


update : Msg -> Model -> (Model, Cmd Msg)
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

    ClearSel ->
      noEffects model


view model =
  Html.div []
    [ selectComponent ["aaa", "bbb", "ccc"]
    , Html.button
      --[ onClick <| AddObject { name = "test" } ]
      [ onClick ClearSel ]
      [ Html.text "Add Object" ]
    , Html.div [] [ Html.text <| toString model ]
    , Html.pre [] (model.files |> List.map (htmlFile model))
    ]


printFile : Model -> File -> String
printFile model file =
  file.context
    |> mapContext
    |> List.map (printFunction model)
    |> String.join "\n\n\n"


htmlFile : Model -> File -> Html Msg
htmlFile model file =
  let xs = file.context
    |> mapContext
    |> List.map (\e -> Html.map (SetExpr e.ref) <| htmlFunction model e)
  in Html.div [] xs


type alias Model =
  { files : List File
  , parent : Dict.Dict ExprRef ExprRef
  , currentRef : Maybe ExprRef
  , currentExpr : Expr
  }


type Context = Context (Array.Array Variable)


emptyContext : Context
emptyContext = Context Array.empty


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
  | ESel Expr -- Current selection


type Symbol -- Unused.
  = SVar Variable
  | STyVar TypeVariable
  | STyCon TypeConstructor


type alias ExprRef = Int


clearSel : Expr -> Expr
clearSel e =
  case e of
    ESel e1 -> e1
    x -> x


mapSel : (Expr -> Expr) -> Expr -> Expr
mapSel f e =
  case e of
    ESel e1 -> f e1
    x -> x


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

    ESel e -> printExpr model e


htmlExpr : Model -> Expr -> Html Expr
htmlExpr model e =
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
        ([ Html.text "[" ] ++ (Array.map (htmlExpr model) ls |> Array.toList) ++ [ Html.text "]" ])

      EIf cond eTrue eFalse ->
        [ Html.text "if"
        , Html.map (\x -> EIf x eTrue eFalse) <| htmlExpr model cond
        , Html.text "then"
        , Html.map (\x -> EIf cond x eFalse) <| htmlExpr model eTrue
        , Html.text "else"
        , Html.map (\x -> EIf cond eTrue x) <| htmlExpr model eFalse
        ]

      EApp e1 e2 ->
        [ Html.text "("
        , Html.map (\x -> EApp x e2) <| htmlExpr model e1
        , Html.map (\x -> EApp e1 x) <| htmlExpr model e2
        , Html.text ")"
        ]

      ESel e ->
        [ Html.text "->"
        , htmlExpr model e
        , Html.text "<-"
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
        [ onClick <| EIf EEmpty EEmpty EEmpty ]
        [ Html.text " [if] " ]
      , Html.a
        [ onClick <| EBool True ]
        [ Html.text " [True] " ]
      , Html.a
        [ onClick <| EBool False ]
        [ Html.text " [False] " ]
      , Html.a
        [ onClick <| EInt 0 ]
        [ Html.text " [0] " ]
      , Html.a
        [ onClick <| EInt 1 ]
        [ Html.text " [1] " ]
      , Html.a
        [ onClick EEmpty ]
        [ Html.text " [x] " ]
      ])


printFunctionSignature : Model -> Variable -> String
printFunctionSignature model f =
  f.name ++ " : " ++ (printType f.type_)


(=>) : String -> String -> (String, String)
(=>) = (,)


htmlFunctionSignature : Model -> Variable -> Html Expr
htmlFunctionSignature model f =
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


htmlFunctionBody : Model -> Variable -> Html Expr
htmlFunctionBody model f =
  Html.div []
    [ Html.text f.name
    , f.context
      |> mapContext
      |> List.map printArg
      |> String.join " "
      |> Html.text
    , Html.text "="
    , htmlExpr model f.value
    ]


printFunction : Model -> Variable -> String
printFunction model f =
  String.join "\n"
    [ (printFunctionSignature model f)
    , (printFunctionBody model f)
    ]


htmlFunction : Model -> Variable -> Html Expr
htmlFunction model v =
  Html.div []
    [ htmlFunctionSignature model v
    , htmlFunctionBody model v
    ]


-- http://ethanschoonover.com/solarized
colorscheme =
  { background = "#fdf6e3"
  , foreground = "#657b83"
  , yellow = "#b58900"
  , orange = "#cb4b16"
  , red = "#dc322f"
  , magenta = "#d33682"
  , violet = "#6c71c4"
  , blue = "#268bd2"
  , cyan = "#2aa198"
  , green = "#859900"
  }


selectComponent : List String -> Html a
selectComponent es =
  Html.div
    [ style
      [ "border-color" => colorscheme.foreground
      , "border-style" => "solid"
      , "width" => "10em"
      , "max-height" => "10em"
      , "overflow" => "auto"
      ]
    ]
    [ selectElement "x"
    , selectElement "if"
    , selectElement "->"
    , selectElement "[]"
    ]


selectElement : String -> Html a
selectElement e =
  Html.div
    [ style
      [ "background-color" => colorscheme.background
      , "color" => colorscheme.foreground
      , "padding" => "2px"
      ]
    ]
    [ Html.text e
    ]
