module Types exposing (..)

import Array
import Dict
import Proto.Ast as Ast
import Proto.Server as Server


type Msg
    = Nop
    | SetCurrentRef (List ExprRef)
    | SetNode Int Node
    | Input String
    | LoadFile
    | LoadFileSuccess Server.GetFileResponse
    | SaveFile


type alias ExprRef =
    Int


type alias Context =
    Dict.Dict ExprRef Node


type alias Model =
    { file :
        Ast.File
        -- Head is the current ref.
    , currentRef : List ExprRef
    , input : String
    }


type Node
    = Expr Ast.Expression
    | VarDef Ast.VariableDefinition
    | Pat Ast.Pattern


type alias Action =
    { label : String
    , msg : Msg
    }
