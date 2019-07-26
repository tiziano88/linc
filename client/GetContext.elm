module GetContext exposing (getContextExpression, getContextFile, getContextNode, getContextPattern, getContextVariableDefinition, getCurrentContext, lookupContext)

import Defaults exposing (..)
import GetNode exposing (..)
import Proto.Ast as Ast
import Types exposing (..)



-- TODO: Use refPath and file context.


getCurrentContext : Model -> Context
getCurrentContext model =
    let
        a =
            List.concat <| List.map getContextNode <| List.filterMap (getNode model) <| List.drop 1 model.refPath

        b =
            getContextFile model.file
    in
    a ++ b


getContextFile : Ast.File -> Context
getContextFile file =
    let
        a =
            List.map (\def -> ( def.ref, VarDef def )) file.variableDefinitions
    in
    a


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


getContextNode : Node -> Context
getContextNode node =
    case node of
        Expr expr ->
            getContextExpression expr

        VarDef def ->
            List.concat <| List.map getContextPattern def.arguments

        --getContextVariableDefinition def
        Pat pat ->
            getContextPattern pat


lookupContext : Context -> ExprRef -> Maybe Node
lookupContext ctx ref =
    List.head <| List.map (\( r, n ) -> n) <| List.filter (\( r, n ) -> r == ref) ctx



--mergeContexts : Context -> List Context -> Context
--mergeContexts ctx ctxs =
--List.foldl Dict.union ctx ctxs
-- mapNode : Node -> (a -> b) ->
-- TODO: traverseWithContext : Context -> Node -> (Context -> Node -> a) -> a
