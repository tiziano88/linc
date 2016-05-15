module Types exposing (..)

import Array
import Dict


type Msg
  = Nop
  | SetCurrentRef ExprRef
  | MapExpr (Variable -> List Variable)



type alias Model =
  { files : List File
  , parent : Dict.Dict ExprRef ExprRef
  , currentRef : Maybe ExprRef
  }


type alias File =
  { name : String
  , nextRef : ExprRef
  , context : List Variable
  }


type alias Variable =
  { name : String
  , ref : ExprRef
  , type_ : Type
  , context : List ExprRef
  , value : Expr
  }


type alias Definition =
  { variable : Variable
  , value : Expr
  }


type alias Node =
  { ref : ExprRef
  , value : Expr
  }


type alias TypeVariable =
  { name : String
  , kind : String -- ?
  }


type alias TypeConstructor =
  { name : String
  }


type Type
  = TEmpty -- Args.
  | TInt
  | TBool
  | TString
  | TList Type
  | TApp Type Type


type Expr
  = EEmpty -- Args.
  | EInt Int
  | EBool Bool
  | EList (Array.Array ExprRef)
  | EString String
  | EIf ExprRef ExprRef ExprRef
  | EApp ExprRef ExprRef


type Symbol -- Unused.
  = SVar Variable
  | STyVar TypeVariable
  | STyCon TypeConstructor


type alias ExprRef = Int
