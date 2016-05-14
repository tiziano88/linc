import Array
import Dict
import Html.App as Html
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode
import String
import Task
import Time

import Print exposing (..)
import Types exposing (..)


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
  , currentOp = identity
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
          , value = EApp 11 100
          }
        , { name = ""
          , ref = 11
          , context = emptyContext
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EBool True
          }
        , { name = ""
          , ref = 100
          , context = emptyContext
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EInt 123
          }
        , { name = "test"
          , ref = 2
          , context = emptyContext
          , type_ = TInt
          , value = EApp 3 4
          }
        , { name = "error"
          , ref = 3
          , context = emptyContext
          , type_ = TInt
          , value = EApp (111) (0)
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
          , value = EList (Array.fromList [0, 1])
          }
        , { name = "cond"
          , ref = 6
          , context = emptyContext
          , type_ = TApp TBool TInt
          , value = EIf 0 100 200
          }
        ]
      }
    ]
  , parent = Dict.empty
  , currentRef = Just 0
  , currentExpr = EEmpty
  , currentOp = identity
  }


noEffects : a -> (a, Cmd b)
noEffects m =
  (m, Cmd.none)


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

    SetCurrentOp f -> noEffects { model | currentOp = f }

view model =
  Html.div []
    [ selectComponent ["aaa", "bbb", "ccc"]
    , Html.button
      [ onClick <| SetCurrentOp (always (EInt 123)) ]
      [ Html.text "123" ]
    , Html.button
      [ onClick <| SetCurrentOp (always (EBool True)) ]
      [ Html.text "True" ]
    , Html.button
      [ onClick <| SetCurrentOp (always (EList Array.empty)) ]
      [ Html.text "[]" ]
    --, Html.button
      --[ onClick <| SetCurrentOp (\e -> (EApp e (-1))) ]
      --[ Html.text "->" ]
    , Html.div [] [ Html.text <| toString model ]
    , Html.pre [] (model.files |> List.map (htmlFile model))
    ]



htmlFile : Model -> File -> Html Msg
htmlFile model file =
  let xs = file.context
    |> mapContext
    |> List.map (\e -> Html.map (SetExpr e.ref) <| htmlFunction model e.ref)
  in Html.div [] xs


htmlExpr : Model -> ExprRef -> Html Expr
htmlExpr model ref =
  case (getVariable model ref) of
    Nothing ->
      Html.text "<<<ERROR>>>"

    Just var ->
      let
        content = case var.value of
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
            , htmlExpr model cond
            , Html.text "then"
            , htmlExpr model eTrue
            , Html.text "else"
            , htmlExpr model eFalse
            ]

          EApp e1 e2 ->
            [ Html.text "("
            , htmlExpr model e1
            , htmlExpr model e2
            , Html.text ")"
            ]

    in
      Html.span
        [ style <|
          [ "border" => "solid"
          , "margin" => "5px"
          , "display" => "inline-block"
          ]
          --(if
            --ref
          --then
            --[ "color" => "blue" ]
          --else
            --[])
        --, onClick' <| model.currentOp e
        ]
        content
        --(content ++
        --[ Html.a
          --[ onClick <| EIf EEmpty EEmpty EEmpty ]
          --[ Html.text " [if] " ]
        --, Html.a
          --[ onClick <| EBool True ]
          --[ Html.text " [True] " ]
        --, Html.a
          --[ onClick <| EBool False ]
          --[ Html.text " [False] " ]
        --, Html.a
          --[ onClick <| EInt 0 ]
          --[ Html.text " [0] " ]
        --, Html.a
          --[ onClick <| EInt 1 ]
          --[ Html.text " [1] " ]
        --, Html.a
          --[ onClick EEmpty ]
          --[ Html.text " [x] " ]
        --])


(=>) : String -> String -> (String, String)
(=>) = (,)


htmlFunctionSignature : Model -> ExprRef -> Html Expr
htmlFunctionSignature model ref =
  case (getVariable model ref) of
    Nothing ->
      Html.text "<<<ERROR>>>"

    Just v ->
      Html.div []
        [ Html.text v.name
        , Html.text " : "
        , Html.text <| (printType v.type_)
        ]


htmlFunctionBody : Model -> ExprRef -> Html Expr
htmlFunctionBody model ref =
  case (getVariable model ref) of
    Nothing ->
      Html.text "<<<ERROR>>>"

    Just v ->
      Html.div []
        [ Html.text v.name
        , v.context
          |> mapContext
          |> List.map printArg
          |> String.join " "
          |> Html.text
        , Html.text "="
        , htmlExpr model ref
        ]



htmlFunction : Model -> ExprRef -> Html Expr
htmlFunction model ref =
  Html.div []
    [ htmlFunctionSignature model ref
    , htmlFunctionBody model ref
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


onClick' a =
  onWithOptions
    "click"
    { defaultOptions | stopPropagation = True}
    (Json.Decode.succeed a)
