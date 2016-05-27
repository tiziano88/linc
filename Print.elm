module Print exposing (..)

import Array
import Dict
import String

import Ast
import Types exposing (..)


printFunctionSignature : Model -> ExprRef -> String
printFunctionSignature model ref =
  case (getVariable model ref) of
    Nothing ->
      "<<<ERROR>>>"

    Just v ->
      v.name ++ " : " -- ++ (printType v.type_)


printFunctionBody : Model -> ExprRef -> String
printFunctionBody model ref =
  case (getVariable model ref) of
    Nothing ->
      "<<<ERROR>>>"

    Just v ->
      String.join " "
        [ v.name
        --, v.context
          --|> List.map (printArg model)
          --|> String.join " "
        , "="
        , printExpr model v
        ]


printFunction : Model -> ExprRef -> String
printFunction model ref =
  String.join "\n"
    [ (printFunctionSignature model ref)
    , (printFunctionBody model ref)
    ]


printFile : Model -> Ast.File -> String
printFile model file =
  file.context
    |> List.map (\v -> v.ref)
    |> List.map (printFunction model)
    |> String.join "\n\n\n"


printArg : Model -> ExprRef -> String
printArg model ref =
  case (getVariable model ref) of
    Nothing ->
      "<<<ERROR>>>"
    Just v ->
      v.name


printType : Ast.Type1 -> String
printType t =
  "xxx"
  --case t of
    --TEmpty -> "<<<EMPTY>>>"
    --TInt -> "Int"
    --TBool -> "Bool"
    --TString -> "String"
    --TList t -> "List " ++ (printType t)
    --TApp t1 t2 -> "(" ++ (printType t1) ++ " -> " ++ (printType t2) ++ ")"


printExpr : Model -> Ast.Expression -> String
printExpr model expr =
  case expr.value of
    Ast.EmptyValue _ ->
      "<<<EMPTY>>>"

    Ast.IntValue v ->
      toString v.value

    Ast.FloatValue v ->
      toString v.value

    Ast.BoolValue v ->
      toString v.value

    Ast.StringValue v ->
      "\"" ++ v.value ++ "\""

    Ast.ListValue v ->
      let
        s =
          v.values
            |> List.map (printExpr model)
            |> String.join ", "
      in
        "[" ++ s ++ "]"

    Ast.IfValue v ->
      String.join " "
        [ "if"
        , printExpr model (Maybe.withDefault defaultExpr v.cond)
        , "then"
        , printExpr model (Maybe.withDefault defaultExpr v.true)
        , "else"
        , printExpr model (Maybe.withDefault defaultExpr v.false)
        ]

    --EApp e1 e2 ->
      --String.join " "
        --[ "(" ++ printExpr model e1
        --, printExpr model e2 ++ ")"
        --]

    _ -> "ooooooooooooooo"

defaultExpr : Ast.Expression
defaultExpr =
  { ref = 888
  , name = "error"
  , type1 = Ast.Xxx 42
  , value = Ast.EmptyValue 42
  }

getVariable : Model -> ExprRef -> Maybe Ast.Expression
getVariable model ref =
  List.filterMap (getExpression ref) model.file.context
    |> List.head


getExpression : ExprRef -> Ast.Expression -> Maybe Ast.Expression
getExpression ref expr =
  if
    expr.ref == ref
  then
    Just expr
  else
    case expr.value of
      Ast.ListValue v ->
        List.filterMap (getExpression ref) v.values |> List.head
      Ast.IfValue v ->
        List.map (Maybe.map <| getExpression ref) [v.cond, v.true, v.false]
          |> List.filterMap identity
          |> List.filterMap identity
          |> List.head
      _ -> Nothing
