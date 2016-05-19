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
  { files = Dict.empty
  , currentFileName = ""
  , parent = Dict.empty
  , currentRef = Nothing
  , input = ""
  }


testModel : Model
testModel =
  { files = buildFiles
    [ { name = "test.elm"
      , nextRef = 888
      , context = buildContext
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
          , value = EApp 111 112
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
          , value = EApp 333 334
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
          , value = EIf 444 445 446
          }
        ]
      }
    ]
  , currentFileName = "test.elm"
  , parent = Dict.empty
  , currentRef = Nothing
  , input = ""
  }


buildContext : List Variable -> Dict.Dict ExprRef Variable
buildContext =
  indexBy (\v -> v.ref)

buildFiles : List File -> Dict.Dict String File
buildFiles =
  indexBy (\v -> v.name)


indexBy : (v -> comparable) -> List v -> Dict.Dict comparable v
indexBy f vs =
  List.foldl (\v -> Dict.insert (f v) v) Dict.empty vs


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

    Input v -> noEffects
      { model
      | input = v
      }

    MapExpr f n ->
      case model.currentRef of
        Nothing -> noEffects model
        Just ref ->
          case getCurrentVariable model of
            Nothing -> noEffects
              { model
              | files =
                model.files
                  |> Dict.update model.currentFileName
                    (Maybe.map <|
                      (\fi ->
                        { fi
                        | context = Dict.insert ref (f { newVariable | ref = ref }) fi.context
                        , nextRef = fi.nextRef + n
                        }))
              }
            Just v -> noEffects
              { model
              | files =
                model.files
                  |> Dict.update model.currentFileName
                    (Maybe.map <|
                      (\fi ->
                        let
                          nf = mapFile ref f fi
                        in
                          { nf | nextRef = nf.nextRef + n}))

              }

mapFile : ExprRef -> (Variable -> Variable) -> File -> File
mapFile ref f file =
  { file
  | context = Dict.update ref (Maybe.map f) file.context
  }


getCurrentFile : Model -> File
getCurrentFile model =
  Dict.get model.currentFileName model.files
    |> Maybe.withDefault { name = "", nextRef = -1, context = Dict.empty }


getCurrentVariable : Model -> Maybe Variable
getCurrentVariable model =
  case model.currentRef of
    Nothing -> Nothing
    Just ref ->
      let
        file = getCurrentFile model
      in
        Dict.get ref file.context


view model =
  let
    file = getCurrentFile model
  in
    Html.div [] <|
      [ selectComponent ["aaa", "bbb", "ccc"]
      , Html.input
        [ onInput Input ]
        []
      , Html.button
        [ onClick <| MapExpr (\v -> { v | name = model.input }) 0 ]
        [ Html.text "setName" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value = EInt 0 }) 0 ]
        [ Html.text "0" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value = EBool False }) 0 ]
        [ Html.text "False" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value = EList Array.empty }) 0 ]
        [ Html.text "[]" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value = EString (model.input) }) 0 ]
        [ Html.text <| "\"" ++ model.input ++ "\" (String) " ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value = EIf (file.nextRef) (file.nextRef + 1) (file.nextRef + 2) }) 3 ]
        [ Html.text "if" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value = EApp (file.nextRef) (file.nextRef + 1) }) 2 ]
        [ Html.text "->" ]
      , Html.button
        [ onClick <| MapExpr (\v -> v) 0 ]
        [ Html.text "x" ]
      ] ++ (modelButtons model file) ++ (typeButtons model file) ++
      [ Html.div [] [ Html.text <| "current file: " ++ model.currentFileName ]
      , Html.div [] [ Html.text <| toString model ]
      , Html.pre [] (model.files |> Dict.values |> List.map (htmlFile model))
      ]

modelButtons model file =
  [intButton, floatButton]
    |> List.concatMap (\x -> x model file)


intButton model file =
  case String.toInt (model.input) of
    Ok n ->
      [ Html.button
        [ onClick <| MapExpr (\v -> { v | value = EInt n }) 0 ]
        [ Html.text <| (toString n) ++ " (Int)" ]
      ]
    _ -> []


floatButton model file =
  case String.toFloat (model.input) of
    Ok n ->
      [ Html.button
        [ onClick <| MapExpr (\v -> { v | value = EFloat n }) 0 ]
        [ Html.text <| (toString n) ++ " (Float)" ]
      ]
    _ -> []

typeButtons model file =
  case getCurrentVariable model of
    Nothing -> []
    Just v ->
      case v.value of
        EInt _ ->
          [ Html.button
            [ onClick <| MapExpr decrement 0 ]
            [ Html.text "-1" ]
          , Html.button
            [ onClick <| MapExpr increment 0 ]
            [ Html.text "+1" ]
          ]

        EBool _ ->
          [ Html.button
            [ onClick <| MapExpr negate 0 ]
            [ Html.text "!" ]
          ]

        EList _ ->
          [ Html.button
            [ onClick <| MapExpr (append file.nextRef) 1 ]
            [ Html.text "append" ]
          ]

        _ -> []


mapVariable : (Expr -> Expr) -> Variable -> Variable
mapVariable f v =
  { v
  | value = f v.value
  }


increment : Variable -> Variable
increment =
  mapVariable <|
    \e -> case e of
      EInt n -> EInt (n + 1)
      _ -> e


decrement : Variable -> Variable
decrement =
  mapVariable <|
    \e -> case e of
      EInt n -> EInt (n - 1)
      _ -> e


negate : Variable -> Variable
negate =
  mapVariable <|
    \e -> case e of
      EBool v -> EBool (not v)
      _ -> e


append : ExprRef -> Variable -> Variable
append ref =
  mapVariable <|
    \e -> case e of
      EList l -> EList (Array.push ref l)
      _ -> e


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
    |> Dict.values
    |> List.filter (\e -> e.name /= "")
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

          EFloat v ->
            [ Html.text <| toString v ]

          EBool v ->
            [ Html.text <| toString v ]

          EString v ->
            [ Html.text <| "\"" ++ v ++ "\"" ]

          EList ls ->
            ([ Html.text "[" ] ++ (Array.map (htmlExpr model) ls |> Array.toList |> List.intersperse (Html.text ",")) ++ [ Html.text "]" ])

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
