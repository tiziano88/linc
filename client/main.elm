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
import Actions exposing (..)
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
          { name = "fib"
          }
        , value = Just
          { ref = 12
          , value = Ast.ExternalRefValue
            { path = "Base"
            , name = "(==)"
            }
          , arguments = Ast.Args
            { values =
              [ { ref = 13
                , value = Ast.IntValue { value = 42 }
                , arguments = Ast.Args { values = [] }
                }
              , { ref = 14
                , value = Ast.RefValue { ref = 2 }
                , arguments = Ast.Args { values = [] }
                }
              ]
            }
          }
        , arguments = [
          { ref = 123
          , pvalue = Ast.LabelValue { name = "n" }
          }
        ]
        }
      , { ref = 2
        , label = Just
          { name = "foo"
          }
        , value = Just
          { ref = 23
          , value = Ast.LambdaValue
            { argument = Just
              { ref = 24
              , pvalue = Ast.LabelValue { name = "iii" }
              }
            , body = Just
              { ref = 42
              , value = Ast.RefValue { ref = 24 }
              , arguments = Ast.Args { values = [] }
              }
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
    context = getCurrentContext model
    actions = case node of
      Nothing -> []
      Just n -> nodeActions model n context
  in
    Html.div [] <|
      [ Html.input
        [ onInput Input ]
        []
      ]
      ++ (List.map actionToButton actions)
      ++
      [ Html.div [] [ Html.text <| toString model ]
      , Html.pre [] [ (htmlFile model node model.file) ]
      , Html.pre [] [ Html.text <| Json.Encode.encode 2 (Ast.fileEncoder model.file) ]
      ]


actionToButton : Action -> Html Msg
actionToButton action =
  Html.button
    [ onClick <| action.msg ]
    [ Html.text action.label ]


htmlFile : Model -> Maybe Node -> Ast.File -> Html Msg
htmlFile model node file =
  let
    newCtx = newContextFile file
    xs =
      file.variableDefinitions
        |> List.map (htmlVariableDefinition model node newCtx)
  in
    Html.div [] xs


htmlExpr : Model -> Maybe Node -> Context -> Ast.Expression -> Html Msg
htmlExpr model node ctx expr =
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
        [ Html.text "[" ]
        ++
        (List.map (htmlExpr model node ctx) ls.values |> List.intersperse (Html.text ","))
        ++
        [ Html.text "]" ]

      Ast.IfValue v ->
        case (v.cond, v.true, v.false) of
          (Just cond, Just true, Just false) ->
            [ Html.text "if"
            , htmlExpr model node ctx cond
            , Html.text "then"
            , htmlExpr model node ctx true
            , Html.text "else"
            , htmlExpr model node ctx false
            ]
          _ -> []

      Ast.LambdaValue v ->
        case (v.argument, v.body) of
          (Just argument, Just body) ->
            let
              newCtx = newContextPattern ctx argument
            in
              [ Html.text "λ"
              , htmlPattern model node ctx argument
              , Html.text "→"
              , htmlExpr model node newCtx body
              ]
          _ -> []

      Ast.RefValue v ->
        [ htmlRef model node ctx v.ref ]

      Ast.ExternalRefValue ref ->
        [ htmlExternalRef model node ctx ref ]


    arguments =
      case expr.arguments of
        Ast.Args a -> List.map (htmlExpr model node ctx) a.values
        _ -> []

  in
    Html.span
      [ style <|
        nodeStyle
        ++
        (if
          Just expr.ref == model.currentRef
        then
          selectedStyle
        else
          [])
        ++
        (if
          isRefSource node expr.ref
        then
          refSourceStyle
        else
          [])
      , onClick' (SetCurrentRef expr.ref)
      ]
      ( case arguments of
          [] ->
            content
          _ ->
            [ Html.text "(" ] ++ content ++ arguments ++ [ Html.text ")" ]
      )


isRefSource : Maybe Node -> ExprRef -> Bool
isRefSource node ref =
  False


isRefTarget : Maybe Node -> ExprRef -> Bool
isRefTarget node ref =
  case node of
    Just n ->
      case n of
        Expr e ->
          case e.value of
            Ast.RefValue v ->
              ref == v.ref
            _ -> False
        _ -> False
    _ -> False


nodeStyle =
  [ "border" => "solid"
  , "margin" => "2px"
  , "padding" => "2px"
  , "display" => "inline-block"
  ]


refStyle =
  [ "border" => "dotted"
  , "margin" => "2px"
  , "padding" => "2px"
  , "display" => "inline-block"
  ]


selectedStyle =
  [ "color" => "blue" ]


refSourceStyle =
  [ "color" => "red" ]


refTargetStyle =
  [ "color" => "orange" ]


htmlRef : Model -> Maybe Node -> Context -> ExprRef -> Html Msg
htmlRef model node ctx ref =
  let
    target = Dict.get ref ctx
  in
    case target of
      Just n ->
        case n of
          Pat pat -> htmlPatternRef model ctx pat
          VarDef def -> htmlVariableDefinitionRef model ctx def
          _ -> Html.text "<<ERROR>>"
      _ -> Html.text "<<ERROR>>"


htmlExternalRef : Model -> Maybe Node -> Context -> Ast.Expression_ExternalRef -> Html Msg
htmlExternalRef model node ctx ref =
  Html.text (ref.path ++ "." ++ ref.name)


(=>) : String -> String -> (String, String)
(=>) = (,)


htmlFunctionSignature : Model -> Context -> Ast.VariableDefinition -> Html Msg
htmlFunctionSignature model ctx def =
  Html.div []
    [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label
    , Html.text " : "
    --, Html.text <| (printType v.type_)
    ]


htmlFunctionBody : Model -> Maybe Node -> Context -> Ast.VariableDefinition -> Html Msg
htmlFunctionBody model node ctx def =
  let
    newCtx = newContextVariableDefinition ctx def
  in
    case def.value of
      Nothing ->
        Html.text "<<<ERROR>>>"

      Just expr ->
        Html.div [] <|
          [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label ]
          ++
          (List.map (htmlPattern model node newCtx) def.arguments)
          ++
          [ Html.text "="
          , htmlExpr model node newCtx expr
          ]


htmlPatternContent : Model -> Context -> Ast.Pattern -> List (Html Msg)
htmlPatternContent model ctx pat =
  case pat.pvalue of
    Ast.LabelValue l ->
      [ Html.text l.name ]
    Ast.TypeConstructorValue v -> []
    Ast.PatternValue v -> []
    Ast.PvalueUnspecified -> []


htmlPattern : Model -> Maybe Node -> Context -> Ast.Pattern -> Html Msg
htmlPattern model node ctx pat =
  let
    content = htmlPatternContent model ctx pat
  in
    Html.div
      [ style <|
        nodeStyle
        ++
        (if
          Just pat.ref == model.currentRef
        then
          selectedStyle
        else
          [])
        ++
        (if
          isRefTarget node pat.ref
        then
          refTargetStyle
        else
          [])
      , onClick' (SetCurrentRef pat.ref)
      ]
      content


htmlPatternRef : Model -> Context -> Ast.Pattern -> Html Msg
htmlPatternRef model ctx pat =
  case pat.pvalue of
    Ast.LabelValue label -> htmlLabelRef label
    _ -> Html.text "<<ERROR>>"


htmlLabelRef : Ast.Label -> Html Msg
htmlLabelRef label =
  Html.div
    [ style refStyle ]
    [ Html.text label.name ]


htmlVariableDefinitionRef : Model -> Context -> Ast.VariableDefinition -> Html Msg
htmlVariableDefinitionRef model ctx def =
  case def.label of
    Just label -> htmlLabelRef label
    _ -> Html.text "<<ERROR>>"


htmlVariableDefinition : Model -> Maybe Node -> Context -> Ast.VariableDefinition -> Html Msg
htmlVariableDefinition model node ctx v =
  Html.div
    [ style <|
      [ "border" => "solid"
      , "margin" => "5px"
      ] ++
      (if
        Just v.ref == model.currentRef
      then
        selectedStyle
      else
        [])
    , onClick' (SetCurrentRef v.ref)
    ]
    [ htmlFunctionSignature model ctx v
    , htmlFunctionBody model node ctx v
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
    { defaultOptions | stopPropagation = True }
    (Json.Decode.succeed a)
