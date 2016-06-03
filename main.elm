module Main exposing (..)

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
import Defaults exposing (..)
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
  }


noEffects : a -> (a, Cmd b)
noEffects m =
  (m, Cmd.none)


update : Msg -> Model -> (Model, Cmd Msg)
update action model =
  let
    currentNode = Debug.log "previous node" <| getCurrentNode model
  in
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

      SetNode n node ->
        case model.currentRef of
          Nothing -> noEffects model
          Just ref ->
            case Debug.log "current node" (getCurrentNode model) of
              Nothing -> noEffects model
              Just v -> noEffects <|
                let
                  fi = model.file
                in
                  { model
                  | file =
                    { fi
                    | variableDefinitions =
                      List.map
                        (setNodeVariableDefinition ref node)
                        fi.variableDefinitions
                    , nextRef = fi.nextRef + n
                    }
                  }


setNodeVariableDefinition : ExprRef -> Node -> Ast.VariableDefinition -> Ast.VariableDefinition
setNodeVariableDefinition ref node def =
  if
    def.ref == ref
  then
    case node of
      VarDef x -> x
      _ -> def
  else
    { def
    | value = Maybe.map (setNodeExpression ref node) def.value
    , arguments = List.map (setNodePattern ref node) def.arguments
    }


setNodeExpression : ExprRef -> Node -> Ast.Expression -> Ast.Expression
setNodeExpression ref node expr =
  if
    expr.ref == ref
  then
    case node of
      Expr e -> e
      _ -> expr
  else
    let newValue =
      case expr.value of
        Ast.IfValue v1 ->
          Ast.IfValue
            { cond = Maybe.map (setNodeExpression ref node) v1.cond
            , true = Maybe.map (setNodeExpression ref node) v1.true
            , false = Maybe.map (setNodeExpression ref node) v1.false
            }

        Ast.ListValue v1 ->
          Ast.ListValue
            { values = (List.map (setNodeExpression ref node) v1.values)
            }

        Ast.LambdaValue v1 ->
          Ast.LambdaValue
            { v1
            | argument = v1.argument
            , body = Maybe.map (setNodeExpression ref node) v1.body
            }

        _ -> expr.value
    in
      { expr
      | value = newValue
      }


setNodePattern : ExprRef -> Node -> Ast.Pattern -> Ast.Pattern
setNodePattern ref node pat =
  if
    pat.ref == ref
  then
    case node of
      Pat p -> p
      _ -> pat
  else
    pat


getCurrentNode : Model -> Maybe Node
getCurrentNode model =
  case model.currentRef of
    Nothing -> Nothing
    Just ref ->
      model.file.variableDefinitions
        |> List.filterMap (getNodeVariableDefinition ref)
        |> List.head


getNodeVariableDefinition : ExprRef -> Ast.VariableDefinition -> Maybe Node
getNodeVariableDefinition ref def =
  if
    def.ref == ref
  then
    Just (VarDef def)
  else
    -- TODO: Find more elegant way.
    Maybe.oneOf <|
      [ def.value `Maybe.andThen` (getNodeExpression ref) ]
      ++ List.map (getNodePattern ref) def.arguments


getNodeExpression : ExprRef -> Ast.Expression -> Maybe Node
getNodeExpression ref expr =
  if
    expr.ref == ref
  then
    Just (Expr expr)
  else
    case expr.value of
      Ast.ListValue v ->
        List.filterMap (getNodeExpression ref) v.values |> List.head

      Ast.IfValue v ->
        List.filterMap (Maybe.map <| getNodeExpression ref) [v.cond, v.true, v.false]
          |> Maybe.oneOf

      Ast.LambdaValue v ->
        List.filterMap (Maybe.map <| getNodeExpression ref) [v.body]
          |> Maybe.oneOf

      _ -> Nothing


getNodePattern : ExprRef -> Ast.Pattern -> Maybe Node
getNodePattern ref pat =
  if
    pat.ref == ref
  then
    Just (Pat pat)
  else
    Nothing


view model =
  let
    file = model.file
    node = getCurrentNode model
    buttons = case node of
      Nothing -> []
      Just n -> nodeButtons model n
  in
    Html.div [] <|
      [ Html.input
        [ onInput Input ]
        []
      ]
      ++ buttons
      ++
      [ Html.div [] [ Html.text <| toString model ]
      , Html.pre [] [ (htmlFile model model.file) ]
      , Html.pre [] [ Html.text <| Json.Encode.encode 2 (Ast.fileEncoder model.file) ]
      ]


nodeButtons : Model -> Node -> List (Html Msg)
nodeButtons model node =
  case node of
    Expr expr -> expressionButtons model expr
    VarDef vdef -> variableDefinitionButtons model vdef
    Pat pat -> patternButtons model pat


expressionButtons : Model -> Ast.Expression -> List (Html Msg)
expressionButtons model expr =
  [ Html.button
    [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = 0 } } ]
    [ Html.text "0" ]
  , Html.button
    [ onClick <| SetNode 0 <| Expr { expr | value = Ast.BoolValue { value = False } } ]
    [ Html.text "False" ]
  , Html.button
    [ onClick <| SetNode 1 <| Expr
      { expr
      | value =
        Ast.ListValue
          { values =
            [ { defaultExpr | ref = model.file.nextRef, value = expr.value } ]
          }
      }
    ]
    [ Html.text "[]" ]
  , Html.button
    [ onClick <| SetNode 3 <| Expr
      { expr
      | value =
        Ast.IfValue
          { cond = Just { defaultExpr | ref = model.file.nextRef, value = expr.value }
          , true = Just { defaultExpr | ref = model.file.nextRef + 1 }
          , false = Just { defaultExpr | ref = model.file.nextRef + 2 }
          }
      }
    ]
    [ Html.text "if" ]
  , Html.button
    [ onClick <| SetNode 3 <| Expr
      { expr
      | value =
        Ast.LambdaValue
          { argument = Nothing
          , body = Just { defaultExpr | ref = model.file.nextRef, value = expr.value }
          }
      }
    ]
    [ Html.text "λ" ]
  , Html.button
    [ onClick <| SetNode 1 <| Expr
      { expr
      | arguments =
        case expr.arguments of
          Ast.Args a -> Ast.Args { values = a.values ++ [ { defaultExpr | ref = model.file.nextRef } ] }
          Ast.ArgumentsUnspecified -> Ast.Args { values = [ { defaultExpr | ref = model.file.nextRef } ] }
      }
    ]
    [ Html.text "arg" ]
  , Html.button
    [ onClick <| SetNode 0 <| Expr expr ]
    [ Html.text "x" ]
  , Html.button
    [ onClick <| SetNode 0 <| Expr { expr | value = Ast.StringValue { value = model.input } } ]
    [ Html.text <| "\"" ++ model.input ++ "\" (String) " ]
  ]
  ++
  case expr.value of
    Ast.StringValue v -> []
    Ast.IntValue v ->
      [ Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v.value - 1 } } ]
        [ Html.text "-1" ]
      , Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v.value + 1 } } ]
        [ Html.text "+1" ]
      ]
    Ast.FloatValue v -> []
    Ast.BoolValue v ->
      [ Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.BoolValue { value = not v.value } } ]
        [ Html.text "!" ]
      ]
    Ast.ListValue v ->
      [ Html.button
        [ onClick <| SetNode 1 <| Expr { expr | value = Ast.ListValue { values = v.values ++ [ { defaultExpr | ref = model.file.nextRef } ] } } ]
        [ Html.text "append" ]
      ]
    _ -> []



variableDefinitionButtons : Model -> Ast.VariableDefinition -> List (Html Msg)
variableDefinitionButtons model def =
  [ Html.button
    [ onClick
      <| SetNode 1
      <| VarDef
        { def
        | arguments =
          def.arguments
          ++ [
            { ref = model.file.nextRef
            , pvalue = Ast.LabelValue { name = "xyz" }
            } ]
        } ]
    [ Html.text "arg" ]
  ]


patternButtons : Model -> Ast.Pattern -> List (Html Msg)
patternButtons model pat =
  case pat.pvalue of
    Ast.LabelValue v ->
      [ Html.button
        [ onClick
          <| SetNode 0
          <| Pat { pat | pvalue = Ast.LabelValue { v | name = model.input } } ]
        [ Html.text "set name" ]
      ]

    _ -> []


intButtons : Model -> Ast.Expression -> List (Html Msg)
intButtons model expr =
  case String.toInt (model.input) of
    Ok n ->
      [ Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = n } } ]
        [ Html.text <| (toString n) ++ " (Int)" ]
      ]
    _ -> []


floatButtons : Model -> Ast.Expression -> List (Html Msg)
floatButtons model expr =
  case String.toFloat (model.input) of
    Ok n ->
      [ Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.FloatValue { value = n } } ]
        [ Html.text <| (toString n) ++ " (Float)" ]
      ]
    _ -> []


htmlFile : Model -> Ast.File -> Html Msg
htmlFile model file =
  let xs = file.variableDefinitions
    |> List.map (htmlVariableDefinition model)
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
        , htmlPattern model (Maybe.withDefault defaultPattern v.argument)
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
      Html.div [] <|
        [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label ]
        ++
        (List.map (htmlPattern model) def.arguments)
        ++
        [ Html.text "="
        , htmlExpr model expr
        ]


htmlPattern : Model -> Ast.Pattern -> Html Msg
htmlPattern model pat =
  let
    content = case pat.pvalue of
      Ast.LabelValue l ->
        [ Html.text l.name ]
      Ast.TypeConstructorValue v -> []
      Ast.PatternValue v -> []
      Ast.PvalueUnspecified -> []
  in
    Html.div
      [ style <|
        [ "border" => "solid"
        , "margin" => "5px"
        , "display" => "inline-block"
        ] ++
        (if
          Just pat.ref == model.currentRef
        then
          [ "color" => "red" ]
        else
          [])
      , onClick' (SetCurrentRef pat.ref)
      ]
      content


htmlVariableDefinition : Model -> Ast.VariableDefinition -> Html Msg
htmlVariableDefinition model v =
  Html.div
    [ style <|
      [ "border" => "solid"
      , "margin" => "5px"
      ] ++
      (if
        Just v.ref == model.currentRef
      then
        [ "color" => "red" ]
      else
        [])
    , onClick' (SetCurrentRef v.ref)
    ]
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
