module Print exposing (..)

import Array
import String

import Types exposing (..)


printFunctionSignature : Model -> ExprRef -> String
printFunctionSignature model ref =
  case (getVariable model ref) of
    Nothing ->
      "<<<ERROR>>>"

    Just v ->
      v.name ++ " : " ++ (printType v.type_)


printFunctionBody : Model -> ExprRef -> String
printFunctionBody model ref =
  case (getVariable model ref) of
    Nothing ->
      "<<<ERROR>>>"

    Just v ->
      String.join " "
        [ v.name
        , v.context
          |> mapContext
          |> List.map printArg
          |> String.join " "
        , "="
        , printExpr model ref
        ]


printFunction : Model -> ExprRef -> String
printFunction model ref =
  String.join "\n"
    [ (printFunctionSignature model ref)
    , (printFunctionBody model ref)
    ]


printFile : Model -> File -> String
printFile model file =
  file.context
    |> List.map (\v -> v.ref)
    |> List.map (printFunction model)
    |> String.join "\n\n\n"


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


printExpr : Model -> ExprRef -> String
printExpr model ref =
  case (getVariable model ref) of
    Nothing ->
      "<<<ERROR>>>"

    Just var ->
      case var.value of
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
                |> Array.map (printExpr model)
                |> Array.toList
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


getVariable : Model -> ExprRef -> Maybe Variable
getVariable model ref =
  model.files
    |> List.map (\x -> getFileFunctionRef x ref)
    |> Maybe.oneOf


getFileFunctionRef : File -> ExprRef -> Maybe Variable
getFileFunctionRef file ref =
  let
    c1 = file.context
    --c2 =
      --file.context
        --|> List.concatMap (\x -> x.context)
    --c = c1 ++ c2
  in
    List.filter (\v -> v.ref == ref) c1 |> List.head

