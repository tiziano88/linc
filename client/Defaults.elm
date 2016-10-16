module Defaults exposing (..)

import Proto.Ast as Ast


defaultExpr : Ast.Expression
defaultExpr =
    { ref = -1
    , value = Ast.EmptyValue 41
    , arguments = Ast.Args { values = [] }
    }


defaultVariableDefinition : Ast.VariableDefinition
defaultVariableDefinition =
    { ref = -1
    , label = Just { name = "◆" }
    , value = Nothing
    , arguments = []
    }


defaultPattern : Ast.Pattern
defaultPattern =
    { ref = -1
    , pvalue = Ast.LabelValue { name = "◆" }
    }
