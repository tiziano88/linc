module Buttons exposing (..)

import Dict
import Html exposing (..)
import Html.Events exposing (..)
import String

import Ast
import Defaults exposing (..)
import Types exposing (..)

nodeButtons : Model -> Node -> Context -> List (Html Msg)
nodeButtons model node ctx =
  case node of
    Expr expr -> expressionButtons model ctx expr
    VarDef vdef -> variableDefinitionButtons model vdef
    Pat pat -> patternButtons model pat


expressionButtons : Model -> Context -> Ast.Expression -> List (Html Msg)
expressionButtons model ctx expr =
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
    [ onClick <| SetNode 2 <| Expr
      { expr
      | value =
        Ast.LambdaValue
          { argument = Just { defaultPattern | ref = model.file.nextRef }
          , body = Just { defaultExpr | ref = model.file.nextRef + 1, value = expr.value }
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
  ( case expr.value of
      Ast.IntValue v ->
        [ Html.button
          [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v.value - 1 } } ]
          [ Html.text "-1" ]
        , Html.button
          [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v.value + 1 } } ]
          [ Html.text "+1" ]
        ]

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
  )
  ++
  (contextButtons ctx expr)


contextButtons : Context -> Ast.Expression -> List (Html Msg)
contextButtons ctx expr =
  ctx
    |> Dict.values
    |> List.map (refButton expr)


refButton : Ast.Expression -> Node -> Html Msg
refButton expr node =
  case node of
    Pat pat ->
      case pat.pvalue of
        Ast.LabelValue l ->
          Html.button
            [ onClick <| SetNode 1 <| Expr { expr | value = Ast.RefValue { ref = pat.ref } } ]
            [ Html.text l.name ]

        _ -> Html.text "x"

    _ -> Html.text "x"


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


