module Types exposing (..)

import Array
import Dict

import Ast


type Msg
  = Nop
  | SetCurrentRef ExprRef
  | MapExpr (Ast.Expression -> Ast.Expression) Int
  | Input String


type alias ExprRef = Int


type alias Model =
  { file : Ast.File
  , currentRef : Maybe ExprRef
  , input : String
  , node : Maybe Node
  }


type Node
  = Expr Ast.Expression
  | VarDef Ast.VariableDefinition
  | TypeAlias Ast.TypeAlias
