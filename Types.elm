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
  { file : File
  , currentRef : Maybe ExprRef
  , input : String
  }


type alias File =
  { name : String
  , nextRef : ExprRef
  , context : List Ast.Expression
  }
