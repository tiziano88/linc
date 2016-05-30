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


testModel : Model
testModel =
  { file =
    { name = "test.elm"
    , nextRef = 888
    , typeAliases =
      [ { ref = 222
        , label = Nothing
        , type1 = Just
          { ref = 223
          , tvalue = Ast.Primitive Ast.Type_Int
          }
        }
      ]
    , variableDefinitions =
      [ { ref = 1
        , label = Just
          { name = "main"
          }
        , value = Just
          { ref = 12
          , value = Ast.IntValue
            { value = 42
            }
          , arguments = Ast.Args { values = [] }
          }
        , arguments = []
        }
      , { ref = 2
        , label = Just
          { name = "mainxxx"
          }
        , value = Just
          { ref = 22
          , value = Ast.IntValue
            { value = 42
            }
          , arguments = Ast.Args { values = [] }
          }
        , arguments = []
        }
      ]
    }
  , currentRef = Nothing
  , input = ""
  , node = Nothing
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
          case Debug.log "current expression" (getCurrentExpression model) of
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
                  | variableDefinitions =
                    fi.variableDefinitions
                      |> List.map (mapVariableDefinition ref f)
                  , nextRef = fi.nextRef + n
                  }
                }


mapVariableDefinition : ExprRef -> (Ast.Expression -> Ast.Expression) -> Ast.VariableDefinition -> Ast.VariableDefinition
mapVariableDefinition ref f def =
  { def
  | value = Maybe.map (mapExpression ref f) def.value
  }


mapExpression : ExprRef -> (Ast.Expression -> Ast.Expression) -> Ast.Expression -> Ast.Expression
mapExpression ref f expr =
  if
    expr.ref == ref
  then
    f expr
  else
    let
      newValue = mapValue ref f expr.value
    in
      { expr
      | value = newValue
      }

mapValue : ExprRef -> (Ast.Expression -> Ast.Expression) -> Ast.Value -> Ast.Value
mapValue ref f value =
  case value of
    Ast.IfValue v1 ->
      Ast.IfValue
        { cond = Maybe.map (mapExpression ref f) v1.cond
        , true = Maybe.map (mapExpression ref f) v1.true
        , false = Maybe.map (mapExpression ref f) v1.false
        }

    Ast.ListValue v1 ->
      Ast.ListValue
        { values = (List.map (mapExpression ref f) v1.values)
        }

    Ast.LambdaValue v1 ->
      Ast.LambdaValue
        { v1
        | argument = Maybe.map (mapExpression ref f) v1.argument
        , body = Maybe.map (mapExpression ref f) v1.body
        }

    _ -> value


--mapArguments : ExprRef -> (Ast.Expression -> Ast.Expression) -> Ast.Arguments -> Ast.Arguments
--mapArguments ref f arguments =
  --case arguments of
    --Ast.Args v1 -> Ast.Args { v1 | values = List.map (mapExpression ref f) v1.values }
    --_ -> arguments


getCurrentExpression : Model -> Maybe Ast.Expression
getCurrentExpression model =
  case model.currentRef of
    Nothing -> Nothing
    Just ref ->
      model.file.variableDefinitions
        |> List.filterMap (getVariableDefinition ref)
        |> List.head
      --getVariable model ref


getVariableDefinition : ExprRef -> Ast.VariableDefinition -> Maybe Ast.Expression
getVariableDefinition ref def =
  case def.value of
    Nothing -> Nothing
    Just expr ->
      getExpression ref expr


view model =
  let
    file = model.file
    expr = Maybe.withDefault defaultExpr <| getCurrentExpression model
  in
    Html.div [] <|
      [ selectComponent ["aaa", "bbb", "ccc"]
      , Html.input
        [ onInput Input ]
        []
      --, Html.button
        --[ onClick <| MapExpr (\v -> { v | name = model.input }) 0 ]
        --[ Html.text "setName" ]
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
  , value = Ast.EmptyValue 41
  , arguments = Ast.Args { values = [] }
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
  case getCurrentExpression model of
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


me : (Ast.Value -> Ast.Value) -> Ast.Expression -> Ast.Expression
me f expr =
  { expr
  | value = f expr.value
  }


increment : Ast.Expression -> Ast.Expression
increment =
  me <|
    \e -> case e of
      Ast.IntValue v -> Ast.IntValue { v | value = v.value + 1 }
      _ -> e


decrement : Ast.Expression -> Ast.Expression
decrement =
  me <|
    \e -> case e of
      Ast.IntValue v -> Ast.IntValue { v | value = v.value - 1 }
      _ -> e


negate : Ast.Expression -> Ast.Expression
negate =
  me <|
    \e -> case e of
      Ast.BoolValue v -> Ast.BoolValue { v | value = not v.value }
      _ -> e


append : ExprRef -> Ast.Expression -> Ast.Expression
append ref =
  me <|
    \e -> case e of
      Ast.ListValue v -> Ast.ListValue { v | values = v.values ++ [ { defaultExpr | ref = ref } ] }
      _ -> e


htmlFile : Model -> Ast.File -> Html Msg
htmlFile model file =
  let xs = file.variableDefinitions
    |> List.map (htmlFunction model)
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
        , htmlExpr model (Maybe.withDefault defaultExpr v.argument)
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


htmlFunctionSignature : Model -> Ast.VariableDefinition -> Html Msg
htmlFunctionSignature model def =
  Html.div []
    [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label
    , Html.text " : "
    --, Html.text <| (printType v.type_)
    ]


htmlFunctionBody : Model -> Ast.VariableDefinition -> Html Msg
htmlFunctionBody model def =
  case def.value of
    Nothing ->
      Html.text "<<<ERROR>>>"

    Just expr ->
      Html.div []
        [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label
        --, expr.context
          --|> List.map (printArg model)
          --|> String.join " "
          --|> Html.text
        , Html.text "="
        , htmlExpr model expr
        ]


htmlFunction : Model -> Ast.VariableDefinition -> Html Msg
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


onClick' a =
  onWithOptions
    "click"
    { defaultOptions | stopPropagation = True}
    (Json.Decode.succeed a)
