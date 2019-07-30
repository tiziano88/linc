module GetNode exposing (getCurrentNode, getNode, getNodeExpression, getNodeFunctionDefinition, getNodeName, getNodeRef, nodeChildren, oneOf)

import Proto.Ast as Ast
import Types exposing (..)


getCurrentNode : Model -> Maybe Node
getCurrentNode model =
    Maybe.andThen (getNode model) <| List.head model.refPath


getNode : Model -> ExprRef -> Maybe Node
getNode model ref =
    model.file.functionDefinitions
        |> List.filterMap (getNodeFunctionDefinition ref)
        |> List.head


getNodeName : Node -> String
getNodeName node =
    case node of
        Expr expr ->
            case expr.value of
                Ast.IntValue v ->
                    String.fromInt v.value

                Ast.FloatValue v ->
                    String.fromFloat v.value

                Ast.StringValue v ->
                    v.value

                Ast.ExternalRefValue v ->
                    v.name

                _ ->
                    ""

        FuncDef funcDef ->
            Maybe.withDefault "" <| Maybe.map .name funcDef.label

        Arg arg ->
            Maybe.withDefault "" <| Maybe.map .name arg.label


getNodeRef : Node -> ExprRef
getNodeRef node =
    case node of
        Expr expr ->
            expr.ref

        FuncDef funcDef ->
            funcDef.ref

        Arg arg ->
            arg.ref


getNodeFunctionDefinition : ExprRef -> Ast.FunctionDefinition -> Maybe Node
getNodeFunctionDefinition ref def =
    if def.ref == ref then
        Just (FuncDef def)

    else
        -- TODO: Find more elegant way.
        oneOf <|
            [ def.body |> Maybe.andThen (getNodeExpression ref) ]



--++ List.map (getNodePattern ref) def.arguments


getNodeExpression : ExprRef -> Ast.Expression -> Maybe Node
getNodeExpression ref expr =
    if expr.ref == ref then
        Just (Expr expr)

    else
        let
            vNode =
                case expr.value of
                    Ast.ListValue v ->
                        List.filterMap (getNodeExpression ref) v.values |> List.head

                    Ast.IfValue v ->
                        List.filterMap (Maybe.map <| getNodeExpression ref) [ v.cond, v.true, v.false ]
                            |> oneOf

                    Ast.FunctionApplicationValue v ->
                        List.filterMap (getNodeExpression ref) v.arguments |> List.head

                    Ast.RefValue _ ->
                        Nothing

                    Ast.ExternalRefValue _ ->
                        Nothing

                    Ast.ValueUnspecified ->
                        Nothing

                    Ast.EmptyValue _ ->
                        Nothing

                    Ast.BoolValue _ ->
                        Nothing

                    Ast.IntValue _ ->
                        Nothing

                    Ast.FloatValue _ ->
                        Nothing

                    Ast.StringValue _ ->
                        Nothing
        in
        oneOf <| [ vNode ]



--getNodePattern : ExprRef -> Ast.Pattern -> Maybe Node
--getNodePattern ref pat =
--if pat.ref == ref then
--Just (Pat pat)
--else
--Nothing


nodeChildren : Node -> List Node
nodeChildren node =
    case node of
        Expr expr ->
            case expr.value of
                Ast.ListValue v ->
                    List.map Expr v.values

                Ast.IfValue v ->
                    List.map Expr <|
                        List.filterMap identity
                            [ v.cond, v.true, v.false ]

                Ast.FunctionApplicationValue v ->
                    List.map Expr v.arguments

                Ast.RefValue _ ->
                    []

                Ast.ExternalRefValue _ ->
                    []

                Ast.EmptyValue _ ->
                    []

                Ast.BoolValue _ ->
                    []

                Ast.IntValue _ ->
                    []

                Ast.FloatValue _ ->
                    []

                Ast.StringValue _ ->
                    []

                Ast.ValueUnspecified ->
                    []

        FuncDef funcDef ->
            List.map Arg funcDef.arguments
                ++ (Maybe.withDefault [] <| Maybe.map (List.singleton << Expr) funcDef.body)

        Arg arg ->
            []



-- Copied from https://github.com/elm-lang/core/commit/5f43ad84532bd4d462edf5c1ec22b7a62352a2db
-- Find a better way.


oneOf : List (Maybe a) -> Maybe a
oneOf maybes =
    case maybes of
        [] ->
            Nothing

        maybe :: rest ->
            case maybe of
                Nothing ->
                    oneOf rest

                Just _ ->
                    maybe
