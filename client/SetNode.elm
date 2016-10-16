module SetNode exposing (..)

import Proto.Ast as Ast
import Types exposing (..)


setNodeVariableDefinition : ExprRef -> Node -> Ast.VariableDefinition -> Ast.VariableDefinition
setNodeVariableDefinition ref node def =
    if def.ref == ref then
        case node of
            VarDef x ->
                x

            _ ->
                def
    else
        { def
            | value = Maybe.map (setNodeExpression ref node) def.value
        }


setNodeExpression : ExprRef -> Node -> Ast.Expression -> Ast.Expression
setNodeExpression ref node expr =
    if expr.ref == ref then
        case node of
            Expr e ->
                e

            _ ->
                expr
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

                    Ast.ApplicationValue v1 ->
                        Ast.ApplicationValue
                            { v1
                                | left = Maybe.map (setNodeExpression ref node) v1.left
                                , right = Maybe.map (setNodeExpression ref node) v1.right
                            }

                    _ ->
                        expr.value
        in
            { expr
                | value = newValue
            }


setNodePattern : ExprRef -> Node -> Ast.Pattern -> Ast.Pattern
setNodePattern ref node pat =
    if pat.ref == ref then
        case node of
            Pat p ->
                p

            _ ->
                pat
    else
        pat
