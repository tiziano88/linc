module GetContext exposing (..)

import Dict

import Ast
import Defaults exposing (..)
import Types exposing (..)


getCurrentContext : Model -> Context
getCurrentContext model =
  case model.currentRef of
    Nothing -> Dict.empty
    Just ref -> getContext model ref Dict.empty


getContext : Model -> ExprRef -> Context -> Context
getContext model ref ctx =
  getContextFile ref ctx model.file


getContextFile : ExprRef -> Context -> Ast.File -> Context
getContextFile ref ctx file =
  mergeContexts ctx <| List.map (getContextVariableDefinition ref ctx) file.variableDefinitions


getContextVariableDefinition : ExprRef -> Context -> Ast.VariableDefinition -> Context
getContextVariableDefinition model ctx def =
  mergeContexts ctx <| List.map getContextPattern def.arguments


getContextExpression : ExprRef -> Context -> Ast.Expression -> Context
getContextExpression ref ctx expr =
  if
    expr.ref == ref
  then
    ctx
  else
    case expr.value of
      Ast.LambdaValue v ->
        mergeContexts ctx [ getContextPattern (Maybe.withDefault defaultPattern v.argument) ]
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
