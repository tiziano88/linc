module Types exposing (Action, Context, ExprRef, Model, Msg(..), Node(..))

import Array
import Dict
import Proto.Ast as Ast
import Proto.Server as Server


type Msg
    = Nop
    | SetRefPath (List ExprRef)
    | SetNode Int Node
    | DeleteNode
    | CreateFunction
    | Input String
    | LoadFile
    | LoadFileSuccess Server.GetFileResponse
    | SaveFile
    | MoveIn
    | MoveOut
    | MoveLeft
    | MoveRight
    | SetColour String


type alias ExprRef =
    Int


type alias Context =
    List ( ExprRef, Node )


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
