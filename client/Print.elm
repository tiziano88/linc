module Print exposing (printArg, printExpr, printFile, printFunction, printFunctionBody, printFunctionSignature, printLabel, printType)

import Array
import Dict
import Proto.Ast as Ast
import String
import Types exposing (..)


printFunctionSignature : Model -> ExprRef -> String
printFunctionSignature model ref =
    ""



--case (getVariable model ref) of
--Nothing ->
--"<<<ERROR>>>"
--Just v ->
--(Maybe.withDefault "" <| Maybe.map printLabel v.label)
--++ " : " -- ++ (printType v.type_)


printFunctionBody : Model -> ExprRef -> String
printFunctionBody model ref =
    ""



--case (getVariable model ref) of
--Nothing ->
--"<<<ERROR>>>"
--Just v ->
--String.join " "
--[ -- Maybe.withDefault "" <| Maybe.map printLabel v.label
----, v.context
----|> List.map (printArg model)
----|> String.join " "
--"="
--, printExpr model v
--]


printFunction : Model -> ExprRef -> String
printFunction model ref =
    ""



--String.join "\n"
--[ (printFunctionSignature model ref)
--, (printFunctionBody model ref)
--]


printLabel : Ast.Label -> String
printLabel label =
    label.name


printFile : Model -> Ast.File -> String
printFile model file =
    ""



--file.context
--|> List.map (\v -> v.ref)
--|> List.map (printFunction model)
--|> String.join "\n\n\n"


printArg : Model -> ExprRef -> String
printArg model ref =
    ""



--case (getVariable model ref) of
--Nothing ->
--"<<<ERROR>>>"
--Just v ->
--case v.label of
--Just l -> l.name
--_ -> "<<<ERROR>>>"


printType : Ast.Type -> String
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
    ""



--case expr.value of
--Ast.EmptyValue _ ->
--"<<<EMPTY>>>"
--Ast.IntValue v ->
--toString v.value
--Ast.FloatValue v ->
--toString v.value
--Ast.BoolValue v ->
--toString v.value
--Ast.StringValue v ->
--"\"" ++ v.value ++ "\""
--Ast.ListValue v ->
--let
--s =
--v.values
--|> List.map (printExpr model)
--|> String.join ", "
--in
--"[" ++ s ++ "]"
--Ast.IfValue v ->
--String.join " "
--[ "if"
--, printExpr model (Maybe.withDefault defaultExpr v.cond)
--, "then"
--, printExpr model (Maybe.withDefault defaultExpr v.true)
--, "else"
--, printExpr model (Maybe.withDefault defaultExpr v.false)
--]
----EApp e1 e2 ->
----String.join " "
----[ "(" ++ printExpr model e1
----, printExpr model e2 ++ ")"
----]
--_ -> "ooooooooooooooo"
