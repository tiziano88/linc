module GetContext exposing (..)

import Proto.Ast as Ast
import Defaults exposing (..)
import Types exposing (..)


-- TODO: Use refPath and file context.


getCurrentContext : Model -> Context
getCurrentContext model =
    []


getContextFile : Ast.File -> Context
getContextFile file =
    List.map (\def -> ( def.ref, VarDef def )) file.variableDefinitions


getContextVariableDefinition : Ast.VariableDefinition -> Context
getContextVariableDefinition def =
    case def.value of
        Just v ->
            getContextExpression v

        _ ->
            []


getContextExpression : Ast.Expression -> Context
getContextExpression expr =
    case expr.value of
        Ast.LambdaValue v ->
            case v.argument of
                Just argument ->
                    getContextPattern argument

                _ ->
                    []

        _ ->
            []


getContextPattern : Ast.Pattern -> Context
getContextPattern pat =
    case pat.pvalue of
        Ast.LabelValue _ ->
            [ ( pat.ref, Pat pat ) ]

        _ ->
            []


lookupContext : Context -> ExprRef -> Maybe Node
lookupContext ctx ref =
    List.head <| List.map (\( r, n ) -> n) <| List.filter (\( r, n ) -> r == ref) ctx



--mergeContexts : Context -> List Context -> Context
--mergeContexts ctx ctxs =
--List.foldl Dict.union ctx ctxs
-- mapNode : Node -> (a -> b) ->
-- TODO: traverseWithContext : Context -> Node -> (Context -> Node -> a) -> a
