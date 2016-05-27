import Array
import Dict
import Html.App as Html
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode
import Json.Encode
import String
import Task
import Time

import Ast
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


--initialModel : Model
--initialModel =
  --{ file = 
  --, currentFileName = ""
  --, parent = Dict.empty
  --, currentRef = Nothing
  --, input = ""
  --}


testModel : Model
testModel =
  { file =
    { name = "test.elm"
    , nextRef = 888
    , context =
      [ { ref = 1
        , name = "main"
        , type1 = Nothing
        , value =
          Ast.IntValue
            { value = 42
            }
        , arguments = Ast.ArgumentsUnspecified
        }
      , { ref = 2
        , name = "xxx"
        , type1 = Nothing
        , value =
          Ast.IntValue
            { value = 42
            }
        , arguments = Ast.ArgumentsUnspecified
        }
      ]
    }
  , currentRef = Nothing
  , input = ""
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

    Input v -> noEffects
      { model
      | input = v
      }

    MapExpr f n ->
      case model.currentRef of
        Nothing -> noEffects model
        Just ref ->
          case Debug.log "model" (getCurrentVariable model) of
            Nothing -> noEffects
              model
              --{ model
              --| files =
                --model.files
                  --|> Dict.update model.currentFileName
                    --(Maybe.map <|
                      --(\fi ->
                        --{ fi
                        --| context = Dict.insert ref (f { newVariable | ref = ref }) fi.context
                        --, nextRef = fi.nextRef + n
                        --}))
              --}
            Just v -> noEffects <|
              let
                fi = model.file
              in
                { model
                | file =
                  { fi
                  | context =
                    fi.context
                      |> List.map (mapExpr ref f)
                  , nextRef = fi.nextRef + n
                  }
                }

mapExpr : ExprRef -> (Ast.Expression -> Ast.Expression) -> Ast.Expression -> Ast.Expression
mapExpr ref f expr =
  if
    expr.ref == ref
  then
    f expr
  else
    let
      newValue = mapValue ref f expr.value
      newArguments = mapArguments ref f expr.arguments
    in
      { expr
      | value = newValue
      , arguments = newArguments
      }

mapValue : ExprRef -> (Ast.Expression -> Ast.Expression) -> Ast.Value -> Ast.Value
mapValue ref f value =
  case value of
    Ast.IfValue v1 ->
      Ast.IfValue
        { cond = Maybe.map (mapExpr ref f) v1.cond
        , true = Maybe.map (mapExpr ref f) v1.true
        , false = Maybe.map (mapExpr ref f) v1.false
        }

    Ast.ListValue v1 ->
      Ast.ListValue
        { values = (List.map (mapExpr ref f) v1.values)
        }

    Ast.LambdaValue v1 ->
      Ast.LambdaValue
        { v1
        | body = Maybe.map (mapExpr ref f) v1.body
        }

    _ -> value


mapArguments : ExprRef -> (Ast.Expression -> Ast.Expression) -> Ast.Arguments -> Ast.Arguments
mapArguments ref f arguments =
  case arguments of
    Ast.Args v1 -> Ast.Args { v1 | values = List.map (mapExpr ref f) v1.values }
    _ -> arguments

--mapArguments : ExprRef -> (Ast.Expression -> Ast.Expression) -> Ast.Arguments -> Ast.Arguments
--mapArguments ref f args =
  --case args of


getCurrentVariable : Model -> Maybe Ast.Expression
getCurrentVariable model =
  case model.currentRef of
    Nothing -> Nothing
    Just ref ->
      getVariable model ref


view model =
  let
    file = model.file
    expr = Maybe.withDefault defaultExpr <| getCurrentVariable model
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
        [ onClick <| MapExpr (\v -> { v | value = Ast.IntValue { value = 0 } }) 0 ]
        [ Html.text "0" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value = Ast.BoolValue { value = False } }) 0 ]
        [ Html.text "False" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value = Ast.ListValue { values = [ { defaultExpr | ref = file.nextRef, value = expr.value } ] } }) 1 ]
        [ Html.text "[]" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value =
          Ast.IfValue
            { cond = Just { defaultExpr | ref = file.nextRef, value = expr.value }
            , true = Just { defaultExpr | ref = file.nextRef + 1 }
            , false = Just { defaultExpr | ref = file.nextRef + 2 }
            }
          }) 3 ]
        [ Html.text "if" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | value =
          Ast.LambdaValue
            { argument = Nothing
            , body = Just { defaultExpr | ref = file.nextRef, value = expr.value }
            }
          }) 1 ]
        [ Html.text "λ" ]
      , Html.button
        [ onClick <| MapExpr (\v -> { v | arguments =
          case v.arguments of
            Ast.Args a -> Ast.Args { values = a.values ++ [ { defaultExpr | ref = file.nextRef } ] }
            Ast.ArgumentsUnspecified -> Ast.Args { values = [ { defaultExpr | ref = file.nextRef } ] }
        }) 1 ]
        [ Html.text "arg" ]
      --, Html.button
        --[ onClick <| MapExpr (\v -> { v | value = EApp (file.nextRef) (file.nextRef + 1) }) 2 ]
        --[ Html.text "->" ]
      , Html.button
        [ onClick <| MapExpr (\v -> v) 0 ]
        [ Html.text "x" ]
      ] ++ (modelButtons model file) ++ (typeButtons model file) ++
      [ Html.div [] [ Html.text <| toString model ]
      , Html.pre [] [ (htmlFile model model.file) ]
      , Html.pre [] [ Html.text <| Json.Encode.encode 2 (Ast.fileEncoder model.file) ]
      ]


defaultExpr : Ast.Expression
defaultExpr =
  { ref = -1
  , name = ""
  , type1 = Nothing
  , value = Ast.EmptyValue 41
  , arguments = Ast.ArgumentsUnspecified
  }


modelButtons model file =
  [stringButtons, intButtons, floatButtons]
    |> List.concatMap (\x -> x model file)


stringButtons model file =
  [ Html.button
    [ onClick <| MapExpr (\v -> { v | value = Ast.StringValue { value = model.input } }) 0 ]
    [ Html.text <| "\"" ++ model.input ++ "\" (String) " ]
  ]


intButtons model file =
  case String.toInt (model.input) of
    Ok n ->
      [ Html.button
        [ onClick <| MapExpr (\v -> { v | value = Ast.IntValue { value = n } }) 0 ]
        [ Html.text <| (toString n) ++ " (Int)" ]
      ]
    _ -> []


floatButtons model file =
  case String.toFloat (model.input) of
    Ok n ->
      [ Html.button
        [ onClick <| MapExpr (\v -> { v | value = Ast.FloatValue { value = n } }) 0 ]
        [ Html.text <| (toString n) ++ " (Float)" ]
      ]
    _ -> []

typeButtons model file =
  case getCurrentVariable model of
    Nothing -> []
    Just v ->
      case v.value of
        Ast.IntValue _ ->
          [ Html.button
            [ onClick <| MapExpr decrement 0 ]
            [ Html.text "-1" ]
          , Html.button
            [ onClick <| MapExpr increment 0 ]
            [ Html.text "+1" ]
          ]

        Ast.BoolValue _ ->
          [ Html.button
            [ onClick <| MapExpr negate 0 ]
            [ Html.text "!" ]
          ]

        Ast.ListValue _ ->
          [ Html.button
            [ onClick <| MapExpr (append file.nextRef) 1 ]
            [ Html.text "append" ]
          ]

        _ -> []


mapVariable : (Ast.Value -> Ast.Value) -> Ast.Expression -> Ast.Expression
mapVariable f expr =
  { expr
  | value = f expr.value
  }


increment : Ast.Expression -> Ast.Expression
increment =
  mapVariable <|
    \e -> case e of
      Ast.IntValue v -> Ast.IntValue { v | value = v.value + 1 }
      _ -> e


decrement : Ast.Expression -> Ast.Expression
decrement =
  mapVariable <|
    \e -> case e of
      Ast.IntValue v -> Ast.IntValue { v | value = v.value - 1 }
      _ -> e


negate : Ast.Expression -> Ast.Expression
negate =
  mapVariable <|
    \e -> case e of
      Ast.BoolValue v -> Ast.BoolValue { v | value = not v.value }
      _ -> e


append : ExprRef -> Ast.Expression -> Ast.Expression
append ref =
  mapVariable <|
    \e -> case e of
      Ast.ListValue v -> Ast.ListValue { v | values = v.values ++ [ { defaultExpr | ref = ref } ] }
      _ -> e


htmlFile : Model -> Ast.File -> Html Msg
htmlFile model file =
  let xs = file.context
    |> List.filter (\e -> e.name /= "")
    |> List.map (\e -> htmlFunction model e.ref)
  in Html.div [] xs


htmlExpr : Model -> Ast.Expression -> Html Msg
htmlExpr model expr =
  let
    content = case expr.value of
      Ast.ValueUnspecified ->
        [ Html.text "<<<EMPTY>>>" ]

      Ast.EmptyValue _ ->
        [ Html.text "<<<EMPTY>>>" ]

      Ast.IntValue v ->
        [ Html.text <| toString v.value ]

      Ast.FloatValue v ->
        [ Html.text <| toString v.value ]

      Ast.BoolValue v ->
        [ Html.text <| toString v.value ]

      Ast.StringValue v ->
        [ Html.text <| "\"" ++ v.value ++ "\"" ]

      Ast.ListValue ls ->
        ([ Html.text "[" ] ++ (List.map (htmlExpr model) ls.values |> List.intersperse (Html.text ",")) ++ [ Html.text "]" ])

      Ast.IfValue v ->
        [ Html.text "if"
        , htmlExpr model (Maybe.withDefault defaultExpr v.cond)
        , Html.text "then"
        , htmlExpr model (Maybe.withDefault defaultExpr v.true)
        , Html.text "else"
        , htmlExpr model (Maybe.withDefault defaultExpr v.false)
        ]

      Ast.LambdaValue v ->
        [ Html.text "λ"
        , Html.text "→"
        , htmlExpr model (Maybe.withDefault defaultExpr v.body)
        ]

      Ast.RefValue v ->
        [ Html.text "ref"
        ]

      --EApp e1 e2 ->
        --[ Html.text "("
        --, htmlExpr model e1
        --, htmlExpr model e2
        --, Html.text ")"
        --]

    arguments =
      case expr.arguments of
        Ast.ArgumentsUnspecified -> []
        Ast.Args a -> List.map (htmlExpr model) a.values

  in
    Html.span
      [ style <|
        [ "border" => "solid"
        , "margin" => "5px"
        , "display" => "inline-block"
        ] ++
        (if
          Just expr.ref == model.currentRef
        then
          [ "color" => "red" ]
        else
          [])
      , onClick' (SetCurrentRef expr.ref)
      ]
      (case arguments of
        [] -> content
        _ -> [ Html.text "(" ] ++ content ++ arguments ++ [ Html.text ")" ]
      )


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
        --, Html.text <| (printType v.type_)
        ]


htmlFunctionBody : Model -> ExprRef -> Html Msg
htmlFunctionBody model ref =
  case (getVariable model ref) of
    Nothing ->
      Html.text "<<<ERROR>>>"

    Just expr ->
      Html.div []
        [ Html.text expr.name
        --, expr.context
          --|> List.map (printArg model)
          --|> String.join " "
          --|> Html.text
        , Html.text "="
        , htmlExpr model expr
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
