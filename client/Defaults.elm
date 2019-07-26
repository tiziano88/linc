module Defaults exposing (defaultExpr, defaultPattern, defaultVariableDefinition)

import Proto.Ast as Ast


defaultExpr : Ast.Expression
defaultExpr =
    { ref = -1
    , value = Ast.EmptyValue 41
    }


defaultVariableDefinition : Ast.VariableDefinition
defaultVariableDefinition =
    { ref = -1
    , label = Just { name = "◆", colour = "white" }
    , value = Nothing
    , arguments = []
    }


defaultPattern : Ast.Pattern
defaultPattern =
    { ref = -1
    , pvalue = Ast.LabelValue { name = "◆", colour = "white" }
    }
