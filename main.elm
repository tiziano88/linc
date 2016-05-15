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
  , currentFileName = ""
  , parent = Dict.empty
  , currentRef = Nothing
  }


testModel : Model
testModel =
  { files =
    [ { name = "test.elm"
      , nextRef = 888
      , context =
        [ { name = "num"
          , ref = 0
          , context = []
          , type_ = TInt
          , value = EInt 42
          }
        , { name = "add"
          , ref = 1
          , context = []
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EApp 11 100
          }
        , { name = ""
          , ref = 11
          , context = []
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EBool True
          }
        , { name = ""
          , ref = 100
          , context = []
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EInt 123
          }
        , { name = "test"
          , ref = 2
          , context = []
          , type_ = TInt
          , value = EApp 3 4
          }
        , { name = "error"
          , ref = 3
          , context = []
          , type_ = TInt
          , value = EApp (111) (0)
          }
        , { name = "st"
          , ref = 4
          , context = []
          , type_ = TString
          , value = EString "test"
          }
        , { name = "list"
          , ref = 5
          , context = []
          , type_ = TList TInt
          , value = EList (Array.fromList [0, 1])
          }
        , { name = "cond"
          , ref = 6
          , context = []
          , type_ = TApp TBool TInt
          , value = EIf 0 100 200
          }
        ]
      }
    ]
  , currentFileName = "test.elm"
  , parent = Dict.empty
  , currentRef = Nothing
  }


noEffects : a -> (a, Cmd b)
noEffects m =
  (m, Cmd.none)


update : Msg -> Model -> (Model, Cmd Msg)
update action model =
  case action of
    Nop -> noEffects model

    SetCurrentRef ref -> noEffects
      { model
      | currentRef = Just ref
      }

    MapExpr f ->
      case model.currentRef of
        Nothing -> noEffects model
        Just ref ->
          case getCurrentVariable model of
            Nothing -> noEffects model
            Just v -> noEffects
              { model
              | files =
                model.files
                  |> List.map (\file ->
                    if
                      (file.name == model.currentFileName)
                    then
                      (mapFile ref f file)
                    else
                      file
                    )
              }

mapFile : ExprRef -> (Variable -> List Variable) -> File -> File
mapFile ref f file =
  { file
  | context =
    file.context
      |> List.concatMap (\v -> (if v.ref == ref then f v else [v]))
  }


getCurrentFile : Model -> Maybe File
getCurrentFile model =
  model.files
    |> List.filter (\f -> f.name == model.currentFileName)
    |> List.head


getCurrentVariable : Model -> Maybe Variable
getCurrentVariable model =
  case model.currentRef of
    Nothing -> Nothing
    Just ref ->
      case getCurrentFile model of
        Nothing -> Nothing
        Just file ->
          file.context
            |> List.filter (\v -> v.ref == ref)
            |> List.head


view model =
  Html.div []
    [ selectComponent ["aaa", "bbb", "ccc"]
    , Html.button
      [ onClick <| MapExpr (\v -> [{ v | value = EInt 123 }]) ]
      [ Html.text "123" ]
    , Html.button
      [ onClick <| MapExpr (\v -> [{ v | value = EBool True }]) ]
      [ Html.text "True" ]
    , Html.button
      [ onClick <| MapExpr (\v -> [{ v | value = EList Array.empty }]) ]
      [ Html.text "[]" ]
    , Html.button
      [ onClick <| MapExpr (\v ->
        [ { v | value = EIf 222 333 444 }
        , { newVariable | ref = 222, value = v.value }
        ])
      ]
      [ Html.text "if" ]
    , Html.button
      [ onClick <| MapExpr (\v ->
        [ { v | value = EApp 222 333 }
        , { newVariable | ref = 222, value = v.value }
        ])
      ]
      [ Html.text "->" ]
    , Html.button
      [ onClick <| MapExpr (\v -> []) ]
      [ Html.text "x" ]
    , Html.button
      [ onClick <| MapExpr (\v -> [ decrement v ]) ]
      [ Html.text "-1" ]
    , Html.button
      [ onClick <| MapExpr (\v -> [ increment v ]) ]
      [ Html.text "+1" ]
    , Html.div [] [ Html.text <| "current file: " ++ model.currentFileName ]
    , Html.div [] [ Html.text <| toString model ]
    , Html.pre [] (model.files |> List.map (htmlFile model))
    ]


increment : Variable -> Variable
increment v =
  { v
  | value =
    case v.value of
      EInt n -> EInt (n + 1)
      x -> x
  }

decrement : Variable -> Variable
decrement v =
  { v
  | value =
    case v.value of
      EInt n -> EInt (n - 1)
      x -> x
  }

newVariable : Variable
newVariable =
  { name = ""
  , ref = 1
  , type_ = TEmpty
  , context = []
  , value = EEmpty
  }


htmlFile : Model -> File -> Html Msg
htmlFile model file =
  let xs = file.context
    |> List.map (\e -> htmlFunction model e.ref)
  in Html.div [] xs


htmlExpr : Model -> ExprRef -> Html Msg
htmlExpr model ref =
  let
    content = case (getVariable model ref) of
      Nothing ->
        [ Html.text "<<<ERROR>>>" ]

      Just var ->
        case var.value of
          EEmpty ->
            [ Html.text "<<<EMPTY>>>" ]

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
        ] ++
        (if
          Just ref == model.currentRef
        then
          [ "color" => "red" ]
        else
          [])
      , onClick' (SetCurrentRef ref)
      ]
      content


(=>) : String -> String -> (String, String)
(=>) = (,)


htmlFunctionSignature : Model -> ExprRef -> Html Msg
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


htmlFunctionBody : Model -> ExprRef -> Html Msg
htmlFunctionBody model ref =
  case (getVariable model ref) of
    Nothing ->
      Html.text "<<<ERROR>>>"

    Just v ->
      Html.div []
        [ Html.text v.name
        , v.context
          |> List.map (printArg model)
          |> String.join " "
          |> Html.text
        , Html.text "="
        , htmlExpr model ref
        ]



htmlFunction : Model -> ExprRef -> Html Msg
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
