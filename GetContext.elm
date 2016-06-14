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
    newCtx =
      file.variableDefinitions
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
    let
      valueCtx =
        case expr.value of
          Ast.IfValue v ->
            case (v.cond, v.true, v.false) of
              (Just c, Just t, Just f) ->
                mergeContexts Dict.empty <| List.map (getContextExpression ref ctx) [ c, t, f ]
              _ -> Dict.empty

          Ast.LambdaValue v ->
            case (v.argument, v.body) of
              (Just a, Just b) ->
                let
                  newCtx = getContextPattern a
                in
                  getContextExpression ref newCtx b
              _ -> Dict.empty
          _ -> Dict.empty
      argsCtx =
        case expr.arguments of
          Ast.Args a ->
            mergeContexts Dict.empty <| List.map (getContextExpression ref ctx) a.values
          _ -> Dict.empty
    in
      Dict.union valueCtx argsCtx


getContextPattern : Ast.Pattern -> Context
getContextPattern pat =
  case pat.pvalue of
    Ast.LabelValue _ ->
      Dict.singleton pat.ref <| Pat pat
    _ -> Dict.empty


mergeContexts : Context -> List Context -> Context
mergeContexts ctx ctxs =
  List.foldl Dict.union ctx ctxs
