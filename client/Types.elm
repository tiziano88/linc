module Types exposing (..)

import Array
import Dict
import Proto.Ast as Ast
import Proto.Server as Server


type Msg
    = Nop
    | SetRefPath (List ExprRef)
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
    { file : Ast.File
    , refPath :
        List ExprRef
        -- Head is the current ref.
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
