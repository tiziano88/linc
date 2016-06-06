module GetContext exposing (..)

import Dict

import Ast
import Defaults exposing (..)
import Types exposing (..)


getCurrentContext : Model -> Context
getCurrentContext model =
  case model.currentRef of
    Nothing -> Dict.empty
    Just ref -> getContextFile ref model.file


getContextFile : ExprRef -> Ast.File -> Context
getContextFile ref file =
  let
    newCtx = file.variableDefinitions
      |> List.map (\def -> (def.ref, VarDef def))
      |> Dict.fromList
  in
    file.variableDefinitions
      |> List.map (getContextVariableDefinition ref newCtx)
      |> mergeContexts Dict.empty


getContextVariableDefinition : ExprRef -> Context -> Ast.VariableDefinition -> Context
getContextVariableDefinition ref ctx def =
  let
    newCtx = mergeContexts ctx <| List.map getContextPattern def.arguments
  in
    case def.value of
      Just v -> getContextExpression ref newCtx v
      _ -> Dict.empty


getContextExpression : ExprRef -> Context -> Ast.Expression -> Context
getContextExpression ref ctx expr =
  if
    (Debug.log "eref" expr.ref) == (Debug.log "ref" ref)
  then
    (Debug.log "ctx" ctx)
  else
    case expr.value of
      Ast.LambdaValue v ->
        case v.argument of
          Just a -> getContextPattern a
          _ -> Dict.empty
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
