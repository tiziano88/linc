import Array
import Dict
import Effects exposing (Effects)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import StartApp
import String
import Task
import Time


main : Signal Html
main =
  app.html


port tasks : Signal (Task.Task Effects.Never ())
port tasks =
  app.tasks


app =
  StartApp.start
    { init = init
    , view = view
    , update = update
    , inputs = []
    }


init : (Model, Effects Action)
init =
  noEffects testModel


initialModel : Model
initialModel =
  { files = []
  , currentRef = Nothing
  }


testModel : Model
testModel =
  { files =
    [ { name = "test.elm"
      , nextRef = 888
      , context =
        Context
        [ (0,
          { name = "num"
          , context = emptyContext
          , type_ = TInt
          , value = EInt 42
          })
        , (1,
          { name = "add"
          , context =
            Context
            [ (11,
              { name = "x"
              , context = emptyContext
              , type_ = TEmpty
              , value = EEmpty })
            , (12,
              { name = "y"
              , context = emptyContext
              , type_ = TEmpty
              , value = EEmpty })
            ]
          , type_ = TApp TInt (TApp TInt TInt)
          , value = EApp (ERef 11) (EInt 100)
          })
        , (2,
          { name = "test"
          , context = emptyContext
          , type_ = TInt
          , value = EApp (EApp (ERef 1) (ERef 0)) (ERef 0)
          })
        , (3,
          { name = "error"
          , context = emptyContext
          , type_ = TInt
          , value = EApp (ERef 111) (ERef 0)
          })
        , (4,
          { name = "st"
          , context = emptyContext
          , type_ = TString
          , value = EString "test"
          })
        , (5,
          { name = "list"
          , context = emptyContext
          , type_ = TList TInt
          , value = EList [(ERef 0), (ERef 1)]
          })
        , (6,
          { name = "cond"
          , context = emptyContext
          , type_ = TApp TBool TInt
          , value = EIf (ERef 0) (EInt 100) (EInt 200)
          })
        ]
      }
    ]
  , currentRef = Just 0
  }


noEffects : a -> (a, Effects b)
noEffects m =
  (m, Effects.none)


type Action
  = Nop


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    --AddObject o ->
      --noEffects { model | objects = Dict.insert model.nextObjRef o model.objects, nextObjRef = model.nextObjRef + 1 }

    Nop -> noEffects model


view address model =
  Html.div []
    [ Html.button
      --[ onClick address <| AddObject { name = "test" } ]
      []
      [ Html.text "Add Object" ]
    , Html.div [] [ Html.text <| toString model ]
    , Html.pre [] (model.files |> List.map (htmlFile model))
    ]


printFile : Model -> File -> String
printFile model file =
  file.context
    |> mapContext
    |> List.map (printFunction model)
    |> String.join "\n\n\n"


htmlFile : Model -> File -> Html
htmlFile model file =
  let xs = file.context
    |> mapContext
    |> List.map (htmlFunction model)
  in Html.div [] xs


type alias Model =
  { files : List File
  , currentRef : Maybe ExprRef
  }


type Context = Context (List (WithRef Variable))


emptyContext : Context
emptyContext = Context []


mapContext : Context -> List Variable
mapContext (Context cs) =
  List.map snd cs


mergeContext : Context -> Context -> Context
mergeContext (Context cs1) (Context cs2) =
  Context (List.append cs1 cs2)


lookupContext : Context -> ExprRef -> Maybe Variable
lookupContext (Context cs) ref =
  cs
    |> List.filter (\(r, _) -> r == ref)
    |> List.head
    |> Maybe.map snd


type alias File =
  { name : String
  , nextRef : ExprRef
  , context : Context
  }


type alias Variable =
  { name : String
  , type_ : Type
  , context : Context
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
  | EList (List Expr)
  | EString String
  | EIf Expr Expr Expr
  | EApp Expr Expr


type Symbol -- Unused.
  = SVar Variable
  | STyVar TypeVariable
  | STyCon TypeConstructor


type alias ExprRef = Int


type alias WithRef a = (ExprRef, a)


getVariable : Model -> ExprRef -> Maybe Variable
getVariable model ref =
  model.files
    |> List.map (\x -> getFileFunctionRef x ref)
    |> Maybe.oneOf


getFileFunctionRef : File -> ExprRef -> Maybe Variable
getFileFunctionRef file ref =
  let
    c1 = file.context
    c2 =
      file.context
        |> mapContext
        |> List.map (\x -> x.context)
        |> List.foldl mergeContext emptyContext
    c = mergeContext c1 c2
  in
    lookupContext c ref


printArg : Variable -> String
printArg a =
  a.name


printType : Type -> String
printType t =
  case t of
    TEmpty -> "<<<EMPTY>>>"
    TInt -> "Int"
    TBool -> "Bool"
    TString -> "String"
    TList t -> "List " ++ (printType t)
    TApp t1 t2 -> "(" ++ (printType t1) ++ " -> " ++ (printType t2) ++ ")"


printExpr : Model -> Expr -> String
printExpr model e =
  case e of
    EEmpty ->
      "<<<EMPTY>>>"

    ERef r ->
      let
        mf = getVariable model r
      in
       case mf of
         Just f -> f.name
         Nothing -> "<<<ERROR>>>"

    EInt v ->
      toString v

    EBool v ->
      toString v

    EString v ->
      "\"" ++ v ++ "\""

    EList ls ->
      let
        s =
          ls
            |> List.map (printExpr model)
            |> String.join ", "
      in
        "[" ++ s ++ "]"

    EIf cond eTrue eFalse ->
      String.join " "
        [ "if"
        , printExpr model cond
        , "then"
        , printExpr model eTrue
        , "else"
        , printExpr model eFalse
        ]

    EApp e1 e2 ->
      String.join " "
        [ "(" ++ printExpr model e1
        , printExpr model e2 ++ ")"
        ]


htmlExpr : Model -> Expr -> Html
htmlExpr model e =
  case e of
    EEmpty ->
      span "<<<EMPTY>>>"

    ERef r ->
      let
        mf = getVariable model r
      in
       case mf of
         Just f -> span f.name
         Nothing -> span "<<<ERROR>>>"

    EInt v ->
      span <| toString v

    EBool v ->
      span <| toString v

    EString v ->
      span <| "\"" ++ v ++ "\""

    EList ls ->
      Html.span []
        ([ span "[" ] ++ (List.map (htmlExpr model) ls) ++ [ span "]" ])

    EIf cond eTrue eFalse ->
      Html.span []
        [ span "if"
        , htmlExpr model cond
        , span "then"
        , htmlExpr model eTrue
        , span "else"
        , htmlExpr model eFalse
        ]

    EApp e1 e2 ->
      Html.span []
        [ span <| "(" ++ printExpr model e1
        , span <| printExpr model e2 ++ ")"
        ]


printFunctionSignature : Model -> Variable -> String
printFunctionSignature model f =
  f.name ++ " : " ++ (printType f.type_)

(=>) : String -> String -> (String, String)
(=>) = (,)

span : String -> Html
span t =
  Html.span
    [ style
      [ "margin" =>  "5px"
      ]
    ]
    [ Html.text t ]


htmlFunctionSignature : Model -> Variable -> Html
htmlFunctionSignature model f =
  Html.div []
    [ span f.name
    , span " : "
    , span <| (printType f.type_) ]



printFunctionBody : Model -> Variable -> String
printFunctionBody model f =
  String.join " "
    [ f.name
    , f.context
      |> mapContext
      |> List.map printArg
      |> String.join " "
    , "="
    , printExpr model f.value
    ]


htmlFunctionBody : Model -> Variable -> Html
htmlFunctionBody model f =
  Html.div []
    [ span f.name
    , f.context
      |> mapContext
      |> List.map printArg
      |> String.join " "
      |> span
    , span "="
    , htmlExpr model f.value
    ]


printFunction : Model -> Variable -> String
printFunction model f =
  String.join "\n"
    [ (printFunctionSignature model f)
    , (printFunctionBody model f)
    ]

htmlFunction : Model -> Variable -> Html
htmlFunction model f =
  Html.div []
    [ htmlFunctionSignature model f
    , htmlFunctionBody model f 
    ]
