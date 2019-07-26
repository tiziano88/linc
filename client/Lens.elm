module Lens exposing (colourOfLabel, colourOfNode, expressionOfNode, fileOfModel, firstWhere, firstWhere_, labelOfVariableDefinition, modifyFirstWhere, nameOfLabel, valueOfExpression, variableDefinitionOfNode, variableDefinitionsOfFile, variableDefinitionsOfModel)

import Monocle.Common exposing (..)
import Monocle.Compose exposing (..)
import Monocle.Lens exposing (..)
import Monocle.Optional exposing (..)
import Monocle.Prism exposing (..)
import Proto.Ast as Ast
import Types exposing (..)


firstWhere : (a -> Bool) -> Optional (List a) a
firstWhere f =
    Optional (firstWhere_ f) (\v l -> modifyFirstWhere f (always v) l)


firstWhere_ : (a -> Bool) -> List a -> Maybe a
firstWhere_ f =
    List.filter f >> List.head


modifyFirstWhere : (a -> Bool) -> (a -> a) -> List a -> List a
modifyFirstWhere f m l =
    case l of
        [] ->
            []

        x :: xs ->
            if f x then
                m x :: xs

            else
                x :: modifyFirstWhere f m xs


fileOfModel : Lens Model Ast.File
fileOfModel =
    Lens .file (\f m -> { m | file = f })


expressionOfNode : Prism Node Ast.Expression
expressionOfNode =
    Prism
        (\n ->
            case n of
                Expr v ->
                    Just v

                _ ->
                    Nothing
        )
        Expr


valueOfExpression : Lens Ast.Expression Ast.Value
valueOfExpression =
    Lens .value (\v e -> { e | value = v })


variableDefinitionsOfFile : Lens Ast.File (List Ast.VariableDefinition)
variableDefinitionsOfFile =
    Lens .variableDefinitions (\v f -> { f | variableDefinitions = v })


variableDefinitionsOfModel : Lens Model (List Ast.VariableDefinition)
variableDefinitionsOfModel =
    Monocle.Lens.compose fileOfModel variableDefinitionsOfFile


colourOfLabel : Lens Ast.Label String
colourOfLabel =
    Lens .colour (\c l -> { l | colour = c })


nameOfLabel : Lens Ast.Label String
nameOfLabel =
    Lens .name (\c l -> { l | name = c })


labelOfVariableDefinition : Optional Ast.VariableDefinition Ast.Label
labelOfVariableDefinition =
    Optional .label (\l v -> { v | label = Just l })


variableDefinitionOfNode : Prism Node Ast.VariableDefinition
variableDefinitionOfNode =
    Prism
        (\n ->
            case n of
                VarDef v ->
                    Just v

                _ ->
                    Nothing
        )
        VarDef


colourOfNode : Optional Node String
colourOfNode =
    fromPrism variableDefinitionOfNode
        |> optionalWithOptional labelOfVariableDefinition
        |> optionalWithOptional (fromLens colourOfLabel)
