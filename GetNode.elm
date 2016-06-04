module GetNode exposing (..)

import Ast
import Types exposing (..)


getCurrentNode : Model -> Maybe Node
getCurrentNode model =
  case model.currentRef of
    Nothing -> Nothing
    Just ref -> getNode model ref


getNode : Model -> Int -> Maybe Node
getNode model ref =
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
        Maybe.oneOf <|
          (List.filterMap (Maybe.map <| getNodeExpression ref) [v.body])
          ++ (List.filterMap (Maybe.map <| getNodePattern ref) [v.argument])

      _ -> Nothing


getNodePattern : ExprRef -> Ast.Pattern -> Maybe Node
getNodePattern ref pat =
  if
    pat.ref == ref
  then
    Just (Pat pat)
  else
    Nothing
