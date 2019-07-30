module Defaults exposing (defaultExpr, defaultFunctionDefinition, defaultPattern)

import Proto.Ast as Ast


defaultExpr : Ast.Expression
defaultExpr =
    { ref = -1
    , value = Ast.EmptyValue 41
    }


defaultFunctionDefinition : Ast.FunctionDefinition
defaultFunctionDefinition =
    { ref = -1
    , label = Just { name = "◆", colour = "white" }
    , arguments = []
    , returnType = Nothing
    , body = Nothing
    }


defaultPattern : Ast.Pattern
defaultPattern =
    { ref = -1
    , pvalue = Ast.LabelValue { name = "◆", colour = "white" }
    }
