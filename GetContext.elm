module GetContext exposing (..)

import Dict

import Ast
import Defaults exposing (..)
import Types exposing (..)


getCurrentContext : Model -> Context
getCurrentContext model =
  case model.currentRef of
    Nothing -> Dict.empty
    Just ref -> getContextFile ref Dict.empty model.file


getContextFile : ExprRef -> Context -> Ast.File -> Context
getContextFile ref ctx file =
  let
    newCtx = file.variableDefinitions
      |> List.map (\def -> (def.ref, VarDef def))
      |> Dict.fromList
  in
    file.variableDefinitions
      |> List.map (getContextVariableDefinition ref newCtx)
      |> mergeContexts ctx


getContextVariableDefinition : ExprRef -> Context -> Ast.VariableDefinition -> Context
getContextVariableDefinition ref ctx def =
  let
    newCtx = mergeContexts ctx <| List.map getContextPattern def.arguments
  in
    getContextExpression ref newCtx (Maybe.withDefault defaultExpr def.value)


getContextExpression : ExprRef -> Context -> Ast.Expression -> Context
getContextExpression ref ctx expr =
  if
    (Debug.log "eref" expr.ref) == (Debug.log "ref" ref)
  then
    ctx
  else
    case expr.value of
      Ast.LambdaValue v ->
        Dict.union ctx <| getContextPattern (Maybe.withDefault defaultPattern v.argument)
      _ -> Dict.empty


getContextPattern : Ast.Pattern -> Context
getContextPattern pat =
  case pat.pvalue of
    Ast.LabelValue _ ->
      Dict.singleton pat.ref <| Pat pat
    _ -> Dict.empty


mergeContexts : Context -> List Context -> Context
mergeContexts ctx ctxs =
  List.foldl Dict.union ctx ctxs
