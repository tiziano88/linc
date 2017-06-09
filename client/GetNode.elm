module GetNode exposing (..)

import Proto.Ast as Ast
import Types exposing (..)


getCurrentNode : Model -> Maybe Node
getCurrentNode model =
    case model.refPath of
        [] ->
            Nothing

        ref :: _ ->
            getNode model ref


getNode : Model -> ExprRef -> Maybe Node
getNode model ref =
    model.file.variableDefinitions
        |> List.filterMap (getNodeVariableDefinition ref)
        |> List.head


getNodeName : Node -> String
getNodeName node =
    case node of
        Expr expr ->
            case expr.value of
                Ast.IntValue v ->
                    toString v.value

                Ast.FloatValue v ->
                    toString v.value

                Ast.StringValue v ->
                    v.value

                Ast.ExternalRefValue v ->
                    v.name

                _ ->
                    ""

        VarDef varDef ->
            case varDef.label of
                Just l ->
                    l.name

                Nothing ->
                    ""

        Pat pat ->
            case pat.pvalue of
                Ast.LabelValue l ->
                    l.name

                _ ->
                    ""


getNodeRef : Node -> ExprRef
getNodeRef node =
    case node of
        Expr expr ->
            expr.ref

        VarDef varDef ->
            varDef.ref

        Pat pat ->
            pat.ref


getNodeVariableDefinition : ExprRef -> Ast.VariableDefinition -> Maybe Node
getNodeVariableDefinition ref def =
    if def.ref == ref then
        Just (VarDef def)
    else
        -- TODO: Find more elegant way.
        oneOf <|
            [ def.value |> Maybe.andThen (getNodeExpression ref) ]
                ++ List.map (getNodePattern ref) def.arguments


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

                    Ast.LambdaValue v ->
                        oneOf <|
                            (List.filterMap (Maybe.map <| getNodeExpression ref) [ v.body ])
                                ++ (List.filterMap (Maybe.map <| getNodePattern ref) [ v.argument ])

                    Ast.ApplicationValue v ->
                        oneOf <|
                            (List.filterMap (Maybe.map <| getNodeExpression ref) [ v.left, v.right ])

                    _ ->
                        Nothing
        in
            oneOf <| [ vNode ]


getNodePattern : ExprRef -> Ast.Pattern -> Maybe Node
getNodePattern ref pat =
    if pat.ref == ref then
        Just (Pat pat)
    else
        Nothing


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

                Ast.LambdaValue v ->
                    List.filterMap identity
                        [ Maybe.map Pat v.argument, Maybe.map Expr v.body ]

                Ast.ApplicationValue v ->
                    List.map Expr <|
                        List.filterMap identity
                            [ v.left, v.right ]

                _ ->
                    []

        _ ->
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
