module Main exposing (..)

import Array
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode
import Json.Encode
import String
import Task
import Time
import Proto.Ast as Ast
import Proto.Server as Server
import Actions exposing (..)
import GetNode exposing (..)
import GetContext exposing (..)
import Defaults exposing (..)
import Persistence exposing (..)
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


init : ( Model, Cmd Msg )
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
              , type1 =
                    Just
                        { ref = 223
                        , tvalue = Ast.Primitive Ast.Type_Int
                        }
              }
            ]
        , variableDefinitions =
            [ { ref = 1
              , label =
                    Just
                        { name = "fib"
                        }
              , value =
                    Just
                        { ref = 12
                        , value =
                            Ast.ApplicationValue
                                { left =
                                    Just
                                        { ref = 2222
                                        , value =
                                            Ast.ExternalRefValue
                                                { path = "Base"
                                                , name = "(==)"
                                                }
                                        }
                                , right =
                                    Just
                                        { ref = 22223
                                        , value = Ast.IntValue { value = 42 }
                                        }
                                }
                        }
              , arguments =
                    [ { ref = 123
                      , pvalue = Ast.LabelValue { name = "n" }
                      }
                    ]
              }
            , { ref = 2
              , label =
                    Just
                        { name = "foo"
                        }
              , value =
                    Just
                        { ref = 23
                        , value =
                            Ast.LambdaValue
                                { argument =
                                    Just
                                        { ref = 24
                                        , pvalue = Ast.LabelValue { name = "iii" }
                                        }
                                , body =
                                    Just
                                        { ref = 42
                                        , value = Ast.RefValue { ref = 24 }
                                        }
                                }
                        }
              , arguments = []
              }
            ]
        }
    , refPath = []
    , input = ""
    }


noEffects : a -> ( a, Cmd b )
noEffects m =
    ( m, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update action model =
    let
        currentNode =
            Debug.log "previous node" <| getCurrentNode model

        currentContext =
            Debug.log "previous context" <| getCurrentContext model
    in
        case Debug.log "action" action of
            Nop ->
                noEffects model

            SetRefPath refPath ->
                let
                    m1 =
                        { model | refPath = refPath }

                    newNode =
                        getCurrentNode m1
                in
                    noEffects
                        { m1
                            | input = Maybe.withDefault "" <| Maybe.map getNodeName newNode
                        }

            Input v ->
                noEffects
                    { model
                        | input = v
                    }

            SetNode n node ->
                case List.head model.refPath of
                    Nothing ->
                        noEffects model

                    Just ref ->
                        case Debug.log "current node" (getCurrentNode model) of
                            Nothing ->
                                noEffects model

                            Just v ->
                                noEffects <|
                                    let
                                        fi =
                                            model.file
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

            DeleteNode ->
                case List.head model.refPath of
                    Nothing ->
                        noEffects model

                    Just ref ->
                        case Debug.log "current node" (getCurrentNode model) of
                            Nothing ->
                                noEffects model

                            Just v ->
                                noEffects <|
                                    let
                                        fi =
                                            model.file
                                    in
                                        { model
                                            | file =
                                                { fi
                                                    | variableDefinitions =
                                                        List.map
                                                            (deleteNodeVariableDefinition ref)
                                                            fi.variableDefinitions
                                                }
                                        }

            CreateFunction ->
                noEffects <|
                    let
                        fi =
                            model.file
                    in
                        { model
                            | file =
                                { fi
                                    | variableDefinitions =
                                        fi.variableDefinitions
                                            ++ [ { defaultVariableDefinition
                                                    | ref = fi.nextRef
                                                    , value = Just { defaultExpr | ref = fi.nextRef + 1 }
                                                 }
                                               ]
                                    , nextRef = fi.nextRef + 2
                                }
                        }

            LoadFile ->
                ( model
                , Http.get Server.getFileResponseDecoder "/LoadFile"
                    |> Task.perform (always Nop) LoadFileSuccess
                )

            LoadFileSuccess s ->
                noEffects <|
                    case Debug.log "c" (Json.Decode.decodeString Ast.fileDecoder (Debug.log "GetFileResponse" s).jsonContent) of
                        Err _ ->
                            model

                        Ok v ->
                            { model
                                | file = v
                            }

            SaveFile ->
                let
                    req =
                        { path = "xx"
                        , jsonContent = (Json.Encode.encode 2 <| Ast.fileEncoder model.file)
                        , elmContent = "zz"
                        }
                in
                    ( model
                    , Http.post
                        Server.getFileResponseDecoder
                        "/SaveFile"
                        (Http.string <| Json.Encode.encode 2 <| Server.updateFileRequestEncoder req)
                        |> Task.perform (always Nop) (always Nop)
                    )


view model =
    let
        file =
            model.file

        node =
            getCurrentNode model

        context =
            getCurrentContext model

        actions =
            case node of
                Nothing ->
                    []

                Just n ->
                    nodeActions model n context
    in
        Html.div [] <|
            [ Html.input
                [ onInput Input
                , value model.input
                ]
                []
            ]
                ++ (List.map actionToButton actions)
                ++ [ Html.div [] [ Html.text <| toString model ]
                   , Html.button
                        [ onClick LoadFile ]
                        [ Html.text "Load" ]
                   , Html.button
                        [ onClick SaveFile ]
                        [ Html.text "Save" ]
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
        newAncestors =
            []

        newCtx =
            getContextFile file

        xs =
            file.variableDefinitions
                |> List.map (htmlVariableDefinition model node newCtx newAncestors)
    in
        Html.div [] xs


htmlExpr : Model -> Maybe Node -> Context -> List ExprRef -> Ast.Expression -> Html Msg
htmlExpr model node ctx ancestors expr =
    let
        newAncestors =
            expr.ref :: ancestors

        newCtx =
            (getContextExpression expr) ++ ctx

        content =
            case expr.value of
                Ast.ValueUnspecified ->
                    [ Html.text "◆" ]

                Ast.EmptyValue _ ->
                    [ Html.text "◆" ]

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
                        ++ (List.map (htmlExpr model node newCtx ancestors) ls.values |> List.intersperse (Html.text ","))
                        ++ [ Html.text "]" ]

                Ast.IfValue v ->
                    case ( v.cond, v.true, v.false ) of
                        ( Just cond, Just true, Just false ) ->
                            [ Html.text "if"
                            , htmlExpr model node newCtx newAncestors cond
                            , Html.text "then"
                            , htmlExpr model node newCtx newAncestors true
                            , Html.text "else"
                            , htmlExpr model node newCtx newAncestors false
                            ]

                        _ ->
                            []

                Ast.LambdaValue v ->
                    case ( v.argument, v.body ) of
                        ( Just argument, Just body ) ->
                            [ Html.text "λ"
                            , htmlPattern model node newCtx newAncestors argument
                            , Html.text "→"
                            , htmlExpr model node newCtx newAncestors body
                            ]

                        _ ->
                            []

                Ast.ApplicationValue v ->
                    case ( v.left, v.right ) of
                        ( Just left, Just right ) ->
                            [ Html.text "("
                            , htmlExpr model node newCtx newAncestors left
                            , Html.text " "
                            , htmlExpr model node newCtx newAncestors right
                            , Html.text ")"
                            ]

                        _ ->
                            []

                Ast.RefValue v ->
                    [ htmlRef model node newCtx newAncestors v.ref ]

                Ast.ExternalRefValue ref ->
                    [ htmlExternalRef model node newCtx newAncestors ref ]

        infix =
            False

        -- TODO: Only if it is actually an operator.
    in
        Html.span
            [ style <|
                nodeStyle
                    ++ (if Just expr.ref == List.head model.refPath then
                            selectedStyle
                        else
                            []
                       )
                    ++ (if isRefSource node expr.ref then
                            refSourceStyle
                        else
                            []
                       )
            , onClick_ (SetRefPath newAncestors)
            ]
            content


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

                        _ ->
                            False

                _ ->
                    False

        _ ->
            False


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


htmlRef : Model -> Maybe Node -> Context -> List ExprRef -> ExprRef -> Html Msg
htmlRef model node ctx ancestors ref =
    let
        target =
            lookupContext ctx ref
    in
        case target of
            Just n ->
                case n of
                    Pat pat ->
                        htmlPatternRef model ctx pat

                    VarDef def ->
                        htmlVariableDefinitionRef model ctx def

                    _ ->
                        Html.text "<<ERROR>>"

            _ ->
                Html.text "<<ERROR>>"


htmlExternalRef : Model -> Maybe Node -> Context -> List ExprRef -> Ast.Expression_ExternalRef -> Html Msg
htmlExternalRef model node ctx ancestors ref =
    Html.text (ref.path ++ "." ++ ref.name)


(=>) : String -> String -> ( String, String )
(=>) =
    (,)


htmlFunctionSignature : Model -> Context -> List ExprRef -> Ast.VariableDefinition -> Html Msg
htmlFunctionSignature model ctx ancestors def =
    Html.div []
        [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label
        , Html.text " : "
          --, Html.text <| (printType v.type_)
        ]


htmlFunctionBody : Model -> Maybe Node -> Context -> List ExprRef -> Ast.VariableDefinition -> Html Msg
htmlFunctionBody model node ctx ancestors def =
    let
        newAncestors =
            def.ref :: ancestors

        newCtx =
            (getContextVariableDefinition def) ++ ctx
    in
        case def.value of
            Nothing ->
                Html.text "<<<ERROR>>>"

            Just expr ->
                Html.div [] <|
                    [ Html.text <| Maybe.withDefault "" <| Maybe.map printLabel def.label ]
                        ++ (List.map (htmlPattern model node newCtx newAncestors) def.arguments)
                        ++ [ Html.text "="
                           , htmlExpr model node newCtx newAncestors expr
                           ]


htmlPatternContent : Model -> Context -> Ast.Pattern -> List (Html Msg)
htmlPatternContent model ctx pat =
    case pat.pvalue of
        Ast.LabelValue l ->
            [ Html.text l.name ]

        Ast.TypeConstructorValue v ->
            []

        Ast.PatternValue v ->
            []

        Ast.PvalueUnspecified ->
            []


htmlPattern : Model -> Maybe Node -> Context -> List ExprRef -> Ast.Pattern -> Html Msg
htmlPattern model node ctx ancestors pat =
    let
        content =
            htmlPatternContent model ctx pat
    in
        Html.div
            [ style <|
                nodeStyle
                    ++ (if Just pat.ref == List.head model.refPath then
                            selectedStyle
                        else
                            []
                       )
                    ++ (if isRefTarget node pat.ref then
                            refTargetStyle
                        else
                            []
                       )
            , onClick_ (SetRefPath (pat.ref :: ancestors))
            , contenteditable True
            , on "input" (Json.Decode.map (rename pat) targetValue)
            ]
            content



-- TODO: Fix cursor jumping.


rename : Ast.Pattern -> String -> Msg
rename pat n =
    case pat.pvalue of
        Ast.LabelValue v ->
            SetNode 0 <| Pat { pat | pvalue = Ast.LabelValue { v | name = n } }

        _ ->
            Nop


targetValue : Json.Decode.Decoder String
targetValue =
    Json.Decode.at [ "target", "innerText" ] Json.Decode.string


htmlPatternRef : Model -> Context -> Ast.Pattern -> Html Msg
htmlPatternRef model ctx pat =
    case pat.pvalue of
        Ast.LabelValue label ->
            htmlLabelRef label

        _ ->
            Html.text "<<ERROR>>"


htmlLabelRef : Ast.Label -> Html Msg
htmlLabelRef label =
    Html.div
        [ style refStyle ]
        [ Html.text label.name ]


htmlVariableDefinitionRef : Model -> Context -> Ast.VariableDefinition -> Html Msg
htmlVariableDefinitionRef model ctx def =
    case def.label of
        Just label ->
            htmlLabelRef label

        _ ->
            Html.text "<<ERROR>>"


htmlVariableDefinition : Model -> Maybe Node -> Context -> List ExprRef -> Ast.VariableDefinition -> Html Msg
htmlVariableDefinition model node ctx ancestors def =
    let
        newCtx =
            (List.concat <| List.map getContextPattern def.arguments) ++ ctx
    in
        Html.div
            [ style <|
                [ "border" => "solid"
                , "margin" => "5px"
                ]
                    ++ (if Just def.ref == List.head model.refPath then
                            selectedStyle
                        else
                            []
                       )
            , onClick_ (SetRefPath (def.ref :: ancestors))
            ]
            [ htmlFunctionSignature model newCtx ancestors def
            , htmlFunctionBody model node newCtx ancestors def
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


onClick_ a =
    onWithOptions
        "click"
        { defaultOptions | stopPropagation = True }
        (Json.Decode.succeed a)
