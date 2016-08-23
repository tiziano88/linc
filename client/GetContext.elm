module GetContext exposing (..)

import Dict
import Proto.Ast as Ast
import Defaults exposing (..)
import Types exposing (..)


getCurrentContext : Model -> Context
getCurrentContext model =
    mergeContexts Dict.empty <| List.map (\ref -> getContextFile ref model.file) model.currentRef


newContextFile : Ast.File -> Context
newContextFile file =
    file.variableDefinitions
        |> List.map (\def -> ( def.ref, VarDef def ))
        |> Dict.fromList


getContextFile : ExprRef -> Ast.File -> Context
getContextFile ref file =
    let
        newCtx =
            newContextFile file
    in
        file.variableDefinitions
            |> List.map (getContextVariableDefinition ref newCtx)
            |> mergeContexts Dict.empty



-- TODO: Flip first two arguments to all new* functions?


newContextVariableDefinition : Context -> Ast.VariableDefinition -> Context
newContextVariableDefinition ctx def =
    List.foldr (flip newContextPattern) ctx def.arguments


getContextVariableDefinition : ExprRef -> Context -> Ast.VariableDefinition -> Context
getContextVariableDefinition ref ctx def =
    let
        newCtx =
            newContextVariableDefinition ctx def
    in
        case def.value of
            Just v ->
                getContextExpression ref newCtx v

            _ ->
                Dict.empty


getContextExpression : ExprRef -> Context -> Ast.Expression -> Context
getContextExpression ref ctx expr =
    if (Debug.log "eref" expr.ref) == (Debug.log "ref" ref) then
        (Debug.log "ctx" ctx)
    else
        let
            valueCtx =
                case expr.value of
                    Ast.IfValue v ->
                        case ( v.cond, v.true, v.false ) of
                            ( Just cond, Just true, Just false ) ->
                                mergeContexts Dict.empty <| List.map (getContextExpression ref ctx) [ cond, true, false ]

                            _ ->
                                Dict.empty

                    Ast.LambdaValue v ->
                        case ( v.argument, v.body ) of
                            ( Just argument, Just body ) ->
                                let
                                    newCtx =
                                        newContextPattern ctx argument
                                in
                                    getContextExpression ref newCtx body

                            _ ->
                                Dict.empty

                    _ ->
                        Dict.empty

            argsCtx =
                case expr.arguments of
                    Ast.Args a ->
                        mergeContexts Dict.empty <| List.map (getContextExpression ref ctx) a.values

                    _ ->
                        Dict.empty
        in
            Dict.union valueCtx argsCtx


newContextPattern : Context -> Ast.Pattern -> Context
newContextPattern ctx pat =
    case pat.pvalue of
        Ast.LabelValue _ ->
            Dict.insert pat.ref (Pat pat) ctx

        _ ->
            ctx


mergeContexts : Context -> List Context -> Context
mergeContexts ctx ctxs =
    List.foldl Dict.union ctx ctxs



-- mapNode : Node -> (a -> b) ->
-- TODO: traverseWithContext : Context -> Node -> (Context -> Node -> a) -> a
