module SetNode exposing (..)

import Proto.Ast as Ast

import Types exposing (..)

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
    let
      newValue =
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
              | argument = Maybe.map (setNodePattern ref node) v1.argument
              , body = Maybe.map (setNodeExpression ref node) v1.body
              }

          _ -> expr.value

      newArgs =
        case expr.arguments of
          Ast.Args args -> List.map (setNodeExpression ref node) args.values
          _ -> []
    in
      { expr
      | value = newValue
      , arguments = Ast.Args { values = newArgs }
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


