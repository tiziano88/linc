module GetContext exposing (..)

import Ast
import Types exposing (..)


getCurrentContext : Model -> List Node
getCurrentContext model =
  case model.currentRef of
    Nothing -> []
    Just ref -> getContext model ref


getContext : Model -> ExprRef -> List Node
getContext model ref =
  model.file.variableDefinitions
    |> List.concatMap (getContextVariableDefinition ref)


getContextVariableDefinition : ExprRef -> Ast.VariableDefinition -> List Node
getContextVariableDefinition model ref = []


getContextExpression : ExprRef -> Ast.Expression -> List Node
getContextExpression ref expr =
  if
    expr.ref == ref
  then
    [Expr expr]
  else
    case expr.value of
      Ast.ListValue v -> List.concatMap (getContextExpression ref) v.values
      Ast.IfValue v -> []
      Ast.LambdaValue v -> []
      _ -> []
