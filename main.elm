module Main exposing (..)

import Array
import Dict
import Html.App as Html
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode
import Json.Encode
import String
import Task
import Time

import Ast
import GetNode exposing (..)
import GetContext exposing (..)
import Defaults exposing (..)
import Print exposing (..)
import SetNode exposing (..)
import Types exposing (..)


main : Program Never
main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = always Sub.none
    }


init : (Model, Cmd Msg)
init =
  noEffects testModel


testModel : Model
testModel =
  { file =
    { name = "test.elm"
    , nextRef = 888
    , typeAliases =
      [ { ref = 222
        , label = Nothing
        , type1 = Just
          { ref = 223
          , tvalue = Ast.Primitive Ast.Type_Int
          }
        }
      ]
    , variableDefinitions =
      [ { ref = 1
        , label = Just
          { name = "main"
          }
        , value = Just
          { ref = 12
          , value = Ast.RefValue
            { ref = 123
            }
          , arguments = Ast.Args
            { values =
              [ { ref = 22
                , value = Ast.IntValue
                  { value = 42
                  }
                , arguments = Ast.Args { values = [] }
                }
              ]
            }
          }
        , arguments = [
          { ref = 123
          , pvalue = Ast.LabelValue { name = "yyy" }
          }
        ]
        }
      , { ref = 2
        , label = Just
          { name = "mainxxx"
          }
        , value = Just
          { ref = 23
          , value = Ast.IntValue
            { value = 42
            }
          , arguments = Ast.Args { values = [] }
          }
        , arguments = []
        }
      ]
    }
  , currentRef = Nothing
  , input = ""
  }


noEffects : a -> (a, Cmd b)
noEffects m =
  (m, Cmd.none)


update : Msg -> Model -> (Model, Cmd Msg)
update action model =
  let
    currentNode = Debug.log "previous node" <| getCurrentNode model
    currentContext = Debug.log "previous context" <| getCurrentContext model
  in
    case action of
      Nop -> noEffects model

      SetCurrentRef ref -> noEffects
        { model
        | currentRef = Just ref
        }

      Input v -> noEffects
        { model
        | input = v
        }

      SetNode n node ->
        case model.currentRef of
          Nothing -> noEffects model
          Just ref ->
            case Debug.log "current node" (getCurrentNode model) of
              Nothing -> noEffects model
              Just v -> noEffects <|
                let
                  fi = model.file
                in
                  { model
                  | file =
                    { fi
                    | variableDefinitions =
                      List.map
                        (setNodeVariableDefinition ref node)
                        fi.variableDefinitions
                    , nextRef = fi.nextRef + n
                    }
                  }

view model =
  let
    file = model.file
    node = getCurrentNode model
    buttons = case node of
      Nothing -> []
      Just n -> nodeButtons model n
  in
    Html.div [] <|
      [ Html.input
        [ onInput Input ]
        []
      ]
      ++ buttons
      ++
      [ Html.div [] [ Html.text <| toString model ]
      , Html.pre [] [ (htmlFile model model.file) ]
      , Html.pre [] [ Html.text <| Json.Encode.encode 2 (Ast.fileEncoder model.file) ]
      ]


nodeButtons : Model -> Node -> List (Html Msg)
nodeButtons model node =
  case node of
    Expr expr -> expressionButtons model expr
    VarDef vdef -> variableDefinitionButtons model vdef
    Pat pat -> patternButtons model pat


expressionButtons : Model -> Ast.Expression -> List (Html Msg)
expressionButtons model expr =
  [ Html.button
    [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = 0 } } ]
    [ Html.text "0" ]
  , Html.button
    [ onClick <| SetNode 0 <| Expr { expr | value = Ast.BoolValue { value = False } } ]
    [ Html.text "False" ]
  , Html.button
    [ onClick <| SetNode 1 <| Expr
      { expr
      | value =
        Ast.ListValue
          { values =
            [ { defaultExpr | ref = model.file.nextRef, value = expr.value } ]
          }
      }
    ]
    [ Html.text "[]" ]
  , Html.button
    [ onClick <| SetNode 3 <| Expr
      { expr
      | value =
        Ast.IfValue
          { cond = Just { defaultExpr | ref = model.file.nextRef, value = expr.value }
          , true = Just { defaultExpr | ref = model.file.nextRef + 1 }
          , false = Just { defaultExpr | ref = model.file.nextRef + 2 }
          }
      }
    ]
    [ Html.text "if" ]
  , Html.button
    [ onClick <| SetNode 2 <| Expr
      { expr
      | value =
        Ast.LambdaValue
          { argument = Just { defaultPattern | ref = model.file.nextRef }
          , body = Just { defaultExpr | ref = model.file.nextRef + 1, value = expr.value }
          }
      }
    ]
    [ Html.text "λ" ]
  , Html.button
    [ onClick <| SetNode 1 <| Expr
      { expr
      | arguments =
        case expr.arguments of
          Ast.Args a -> Ast.Args { values = a.values ++ [ { defaultExpr | ref = model.file.nextRef } ] }
          Ast.ArgumentsUnspecified -> Ast.Args { values = [ { defaultExpr | ref = model.file.nextRef } ] }
      }
    ]
    [ Html.text "arg" ]
  , Html.button
    [ onClick <| SetNode 0 <| Expr expr ]
    [ Html.text "x" ]
  , Html.button
    [ onClick <| SetNode 0 <| Expr { expr | value = Ast.StringValue { value = model.input } } ]
    [ Html.text <| "\"" ++ model.input ++ "\" (String) " ]
  ]
  ++
  case expr.value of
    Ast.StringValue v -> []
    Ast.IntValue v ->
      [ Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v.value - 1 } } ]
        [ Html.text "-1" ]
      , Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v.value + 1 } } ]
        [ Html.text "+1" ]
      ]
    Ast.FloatValue v -> []
    Ast.BoolValue v ->
      [ Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.BoolValue { value = not v.value } } ]
        [ Html.text "!" ]
      ]
    Ast.ListValue v ->
      [ Html.button
        [ onClick <| SetNode 1 <| Expr { expr | value = Ast.ListValue { values = v.values ++ [ { defaultExpr | ref = model.file.nextRef } ] } } ]
        [ Html.text "append" ]
      ]
    _ -> []



variableDefinitionButtons : Model -> Ast.VariableDefinition -> List (Html Msg)
variableDefinitionButtons model def =
  [ Html.button
    [ onClick
      <| SetNode 1
      <| VarDef
        { def
        | arguments =
          def.arguments
          ++ [
            { ref = model.file.nextRef
            , pvalue = Ast.LabelValue { name = "xyz" }
            } ]
        } ]
    [ Html.text "arg" ]
  ]


patternButtons : Model -> Ast.Pattern -> List (Html Msg)
patternButtons model pat =
  case pat.pvalue of
    Ast.LabelValue v ->
      [ Html.button
        [ onClick
          <| SetNode 0
          <| Pat { pat | pvalue = Ast.LabelValue { v | name = model.input } } ]
        [ Html.text "set name" ]
      ]

    _ -> []


intButtons : Model -> Ast.Expression -> List (Html Msg)
intButtons model expr =
  case String.toInt (model.input) of
    Ok n ->
      [ Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.IntValue { value = n } } ]
        [ Html.text <| (toString n) ++ " (Int)" ]
      ]
    _ -> []


floatButtons : Model -> Ast.Expression -> List (Html Msg)
floatButtons model expr =
  case String.toFloat (model.input) of
    Ok n ->
      [ Html.button
        [ onClick <| SetNode 0 <| Expr { expr | value = Ast.FloatValue { value = n } } ]
        [ Html.text <| (toString n) ++ " (Float)" ]
      ]
    _ -> []


htmlFile : Model -> Ast.File -> Html Msg
htmlFile model file =
  let xs = file.variableDefinitions
    |> List.map (htmlVariableDefinition model Dict.empty)
  in Html.div [] xs


htmlExpr : Model -> Context -> Ast.Expression -> Html Msg
htmlExpr model ctx expr =
  let
    content = case expr.value of
      Ast.ValueUnspecified ->
        [ Html.text "<<<EMPTY>>>" ]

      Ast.EmptyValue _ ->
        [ Html.text "<<<EMPTY>>>" ]

      Ast.IntValue v ->
        [ Html.text <| toString v.value ]

      Ast.FloatValue v ->
        [ Html.text <| toString v.value ]

      Ast.BoolValue v ->
        [ Html.text <| toString v.value ]

      Ast.StringValue v ->
        [ Html.text <| "\"" ++ v.value ++ "\"" ]

      Ast.ListValue ls ->
        ([ Html.text "[" ] ++ (List.map (htmlExpr model ctx) ls.values |> List.intersperse (Html.text ",")) ++ [ Html.text "]" ])

      Ast.IfValue v ->
        [ Html.text "if"
        , htmlExpr model ctx (Maybe.withDefault defaultExpr v.cond)
        , Html.text "then"
        , htmlExpr model ctx (Maybe.withDefault defaultExpr v.true)
        , Html.text "else"
        , htmlExpr model ctx (Maybe.withDefault defaultExpr v.false)
        ]

      Ast.LambdaValue v ->
        [ Html.text "λ"
        , htmlPattern model ctx (Maybe.withDefault defaultPattern v.argument)
        , Html.text "→"
        , htmlExpr model ctx (Maybe.withDefault defaultExpr v.body)
        ]

      Ast.RefValue v ->
        [ htmlRef model ctx v.ref
        ]

      --EApp e1 e2 ->
        --[ Html.text "("
        --, htmlExpr model e1
        --, htmlExpr model e2
        --, Html.text ")"
        --]

    arguments =
      case expr.arguments of
        Ast.ArgumentsUnspecified -> []
        Ast.Args a -> List.map (htmlExpr model ctx) a.values

  in
    Html.span
      [ style <|
        [ "border" => "solid"
        , "margin" => "5px"
        , "display" => "inline-block"
        ] ++
        (if
          Just expr.ref == model.currentRef
        then
          [ "color" => "red" ]
        else
          [])
      , onClick' (SetCurrentRef expr.ref)
      ]
      (case arguments of
        [] -> content
        _ -> [ Html.text "(" ] ++ content ++ arguments ++ [ Html.text ")" ]
      )


htmlRef : Model -> Context -> ExprRef -> Html Msg
htmlRef model ctx ref =
  let
    n = getNode model ref Dict.empty
  in
    case n of
      Just n ->
        case n of
          Pat p -> htmlPattern model ctx p
          _ -> Html.text "<<ERROR>>"
      Nothing -> Html.text "<<ERROR>>"


(=>) : String -> String -> (String, String)
(=>) = (,)


htmlFunctionSignature : Model -> Context -> Ast.VariableDefinition -> Html Msg
htmlFunctionSignature model ctx def =
  Html.div []
    [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label
    , Html.text " : "
    --, Html.text <| (printType v.type_)
    ]


htmlFunctionBody : Model -> Context -> Ast.VariableDefinition -> Html Msg
htmlFunctionBody model ctx def =
  case def.value of
    Nothing ->
      Html.text "<<<ERROR>>>"

    Just expr ->
      Html.div [] <|
        [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label ]
        ++
        (List.map (htmlPattern model ctx) def.arguments)
        ++
        [ Html.text "="
        , htmlExpr model ctx expr
        ]


htmlPattern : Model -> Context -> Ast.Pattern -> Html Msg
htmlPattern model ctx pat =
  let
    content = case pat.pvalue of
      Ast.LabelValue l ->
        [ Html.text l.name ]
      Ast.TypeConstructorValue v -> []
      Ast.PatternValue v -> []
      Ast.PvalueUnspecified -> []
  in
    Html.div
      [ style <|
        [ "border" => "solid"
        , "margin" => "5px"
        , "display" => "inline-block"
        ] ++
        (if
          Just pat.ref == model.currentRef
        then
          [ "color" => "red" ]
        else
          [])
      , onClick' (SetCurrentRef pat.ref)
      ]
      content


htmlVariableDefinition : Model -> Context -> Ast.VariableDefinition -> Html Msg
htmlVariableDefinition model ctx v =
  Html.div
    [ style <|
      [ "border" => "solid"
      , "margin" => "5px"
      ] ++
      (if
        Just v.ref == model.currentRef
      then
        [ "color" => "red" ]
      else
        [])
    , onClick' (SetCurrentRef v.ref)
    ]
    [ htmlFunctionSignature model ctx v
    , htmlFunctionBody model ctx v
    ]


-- http://ethanschoonover.com/solarized
colorscheme =
  { background = "#fdf6e3"
  , foreground = "#657b83"
  , yellow = "#b58900"
  , orange = "#cb4b16"
  , red = "#dc322f"
  , magenta = "#d33682"
  , violet = "#6c71c4"
  , blue = "#268bd2"
  , cyan = "#2aa198"
  , green = "#859900"
  }


selectComponent : List String -> Html a
selectComponent es =
  Html.div
    [ style
      [ "border-color" => colorscheme.foreground
      , "border-style" => "solid"
      , "width" => "10em"
      , "max-height" => "10em"
      , "overflow" => "auto"
      ]
    ]
    [ selectElement "x"
    , selectElement "if"
    , selectElement "->"
    , selectElement "[]"
    ]


selectElement : String -> Html a
selectElement e =
  Html.div
    [ style
      [ "background-color" => colorscheme.background
      , "color" => colorscheme.foreground
      , "padding" => "2px"
      ]
    ]
    [ Html.text e
    ]


onClick' a =
  onWithOptions
    "click"
    { defaultOptions | stopPropagation = True}
    (Json.Decode.succeed a)
