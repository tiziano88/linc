module Types exposing (..)

import Array
import Dict

import Ast


type Msg
  = Nop
  | SetCurrentRef ExprRef
  | SetNode Int Node
  | Input String


type alias ExprRef = Int


type alias Model =
  { file : Ast.File
  , currentRef : Maybe ExprRef
  , input : String
  }


type Node
  = Expr Ast.Expression
  | VarDef Ast.VariableDefinition
  | Pat Ast.Pattern
