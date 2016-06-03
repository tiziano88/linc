module Defaults exposing (..)

import Ast


defaultExpr : Ast.Expression
defaultExpr =
  { ref = -1
  , value = Ast.EmptyValue 41
  , arguments = Ast.Args { values = [] }
  }


defaultPattern : Ast.Pattern
defaultPattern =
  { ref = -1
  , pvalue = Ast.PvalueUnspecified
  }
