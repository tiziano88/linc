module Types exposing (..)

import Array
import Dict


type Msg
  = Nop
  | SetCurrentRef ExprRef
  | SetExpr ExprRef Expr
  | SetCurrentOp (Expr -> Expr)


type alias Model =
  { files : List File
  , parent : Dict.Dict ExprRef ExprRef
  , currentRef : Maybe ExprRef
  , currentExpr : Expr
  , currentOp : Expr -> Expr
  }


type alias File =
  { name : String
  , nextRef : ExprRef
  , context : Context
  }


type alias Variable =
  { name : String
  , ref : ExprRef
  , type_ : Type
  , context : Context
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
  | ERef ExprRef
  | EInt Int
  | EBool Bool
  | EList (Array.Array Expr)
  | EString String
  | EIf Expr Expr Expr
  | EApp Expr Expr


type Symbol -- Unused.
  = SVar Variable
  | STyVar TypeVariable
  | STyCon TypeConstructor


type alias ExprRef = Int

type Context = Context (Array.Array Variable)

emptyContext : Context
emptyContext = Context Array.empty


mapContext : Context -> List Variable
mapContext (Context cs) =
  Array.toList cs


mergeContext : Context -> Context -> Context
mergeContext (Context cs1) (Context cs2) =
  Context (Array.append cs1 cs2)


lookupContext : Context -> ExprRef -> Maybe Variable
lookupContext (Context cs) ref =
  cs
    |> Array.filter (\v -> v.ref == ref)
    |> Array.get 0


updateContext : Context -> ExprRef -> Expr -> Context
updateContext (Context cs) ref e =
  cs
    |> Array.map (\v -> if v.ref == ref then {v | value = e } else v)
    |> Context



