module Actions exposing (..)

import GetNode exposing (..)
import Html exposing (..)
import Html.Events exposing (..)
import List.Extra
import String
import Proto.Ast as Ast
import Defaults exposing (..)
import Types exposing (..)


nodeActions : Model -> Node -> Context -> List Action
nodeActions model node ctx =
    let
        a =
            case node of
                Expr expr ->
                    expressionActions model ctx expr

                VarDef vdef ->
                    variableDefinitionActions model vdef

                Pat pat ->
                    patternActions model pat

        parent =
            Debug.log "parent" <|
                case List.head <| List.drop 1 <| model.refPath of
                    Just p ->
                        case getNode model p of
                            Just p ->
                                Just p

                            Nothing ->
                                Nothing

                    Nothing ->
                        Nothing

        children =
            Debug.log "children" <| nodeChildren node

        siblings =
            Debug.log "siblings" <|
                case parent of
                    Just p ->
                        nodeChildren p

                    Nothing ->
                        []

        nodeIndex =
            Debug.log "index"
                (siblings
                    |> List.Extra.findIndex (\n -> (getNodeRef n) == (getNodeRef node))
                )

        b =
            [ { label = "↑"
              , msg =
                    -- Remove the head of the refPath, which is the current node.
                    SetRefPath <| List.drop 1 model.refPath
              }
            , { label = "↓"
              , msg =
                    -- Move the first child, if any.
                    case List.head children of
                        Just c ->
                            SetRefPath <| (getNodeRef c) :: model.refPath

                        Nothing ->
                            Nop
              }
            , { label = "←"
              , msg =
                    case nodeIndex of
                        Just i ->
                            case (List.Extra.getAt (i - 1) siblings) of
                                Just c ->
                                    SetRefPath <| (getNodeRef c) :: (List.drop 1 model.refPath)

                                Nothing ->
                                    Nop

                        Nothing ->
                            Nop
              }
            , { label = "→"
              , msg =
                    case nodeIndex of
                        Just i ->
                            case (List.Extra.getAt (i + 1) siblings) of
                                Just c ->
                                    SetRefPath <| (getNodeRef c) :: (List.drop 1 model.refPath)

                                Nothing ->
                                    Nop

                        Nothing ->
                            Nop
              }
            , { label = "Create function"
              , msg = CreateFunction
              }
            ]
    in
        a ++ b


expressionActions : Model -> Context -> Ast.Expression -> List Action
expressionActions model ctx expr =
    [ { label = "0"
      , msg = SetNode 0 <| Expr { expr | value = Ast.IntValue { value = 0 } }
      }
    , { label = "False"
      , msg = SetNode 0 <| Expr { expr | value = Ast.BoolValue { value = False } }
      }
    , { label = "[]"
      , msg =
            SetNode 1 <|
                Expr
                    { expr
                        | value =
                            Ast.ListValue
                                { values = []
                                }
                    }
      }
    , { label = "[◇]"
      , msg =
            SetNode 1 <|
                Expr
                    { expr
                        | value =
                            Ast.ListValue
                                { values =
                                    [ { defaultExpr | ref = model.file.nextRef, value = expr.value } ]
                                }
                    }
      }
    , { label = "if ◆ then ◆ else ◆"
      , msg =
            SetNode 3 <|
                Expr
                    { expr
                        | value =
                            Ast.IfValue
                                { cond = Just { defaultExpr | ref = model.file.nextRef }
                                , true = Just { defaultExpr | ref = model.file.nextRef + 1 }
                                , false = Just { defaultExpr | ref = model.file.nextRef + 2 }
                                }
                    }
      }
    , { label = "if ◇ then ◆ else ◆"
      , msg =
            SetNode 3 <|
                Expr
                    { expr
                        | value =
                            Ast.IfValue
                                { cond =
                                    Just
                                        { defaultExpr
                                            | ref = model.file.nextRef
                                            , value = expr.value
                                        }
                                , true = Just { defaultExpr | ref = model.file.nextRef + 1 }
                                , false = Just { defaultExpr | ref = model.file.nextRef + 2 }
                                }
                    }
      }
    , { label = "if ◆ then ◇ else ◆"
      , msg =
            SetNode 3 <|
                Expr
                    { expr
                        | value =
                            Ast.IfValue
                                { cond = Just { defaultExpr | ref = model.file.nextRef }
                                , true =
                                    Just
                                        { defaultExpr
                                            | ref = model.file.nextRef + 1
                                            , value = expr.value
                                        }
                                , false = Just { defaultExpr | ref = model.file.nextRef + 2 }
                                }
                    }
      }
    , { label = "if ◆ then ◆ else ◇"
      , msg =
            SetNode 3 <|
                Expr
                    { expr
                        | value =
                            Ast.IfValue
                                { cond = Just { defaultExpr | ref = model.file.nextRef }
                                , true = Just { defaultExpr | ref = model.file.nextRef + 1 }
                                , false =
                                    Just
                                        { defaultExpr
                                            | ref = model.file.nextRef + 2
                                            , value = expr.value
                                        }
                                }
                    }
      }
    , { label = "λ ◆ → ◆"
      , msg =
            SetNode 2 <|
                Expr
                    { expr
                        | value =
                            Ast.LambdaValue
                                { argument = Just { defaultPattern | ref = model.file.nextRef }
                                , body = Just { defaultExpr | ref = model.file.nextRef + 1 }
                                }
                    }
      }
    , { label = "λ ◆ → ◇"
      , msg =
            SetNode 2 <|
                Expr
                    { expr
                        | value =
                            Ast.LambdaValue
                                { argument = Just { defaultPattern | ref = model.file.nextRef }
                                , body = Just { defaultExpr | ref = model.file.nextRef + 1, value = expr.value }
                                }
                    }
      }
    , { label = "◇ ◆"
      , msg =
            SetNode 2 <|
                Expr
                    { ref = model.file.nextRef
                    , value =
                        Ast.ApplicationValue
                            { left = Just expr
                            , right = Just { defaultExpr | ref = model.file.nextRef + 1 }
                            }
                    }
      }
    , { label =
            "◆ ◇"
            -- TODO
      , msg =
            SetNode 2 <|
                Expr
                    { ref = model.file.nextRef
                    , value =
                        Ast.ApplicationValue
                            { left = Just { defaultExpr | ref = model.file.nextRef + 1 }
                            , right = Just expr
                            }
                    }
      }
    , { label = "◆"
      , msg = SetNode 0 <| Expr defaultExpr
      }
    , { label = "⌧"
      , msg = DeleteNode
      }
    , { label = "\"" ++ model.input ++ "\" (String) "
      , msg = SetNode 0 <| Expr { expr | value = Ast.StringValue { value = model.input } }
      }
    ]
        ++ (case expr.value of
                Ast.IntValue v ->
                    [ { label = "-1"
                      , msg = SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v.value - 1 } }
                      }
                    , { label = "+1"
                      , msg = SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v.value + 1 } }
                      }
                    ]

                Ast.BoolValue v ->
                    [ { label = "negate"
                      , msg = SetNode 0 <| Expr { expr | value = Ast.BoolValue { value = not v.value } }
                      }
                    ]

                Ast.ListValue v ->
                    [ { label = "append"
                      , msg = SetNode 1 <| Expr { expr | value = Ast.ListValue { values = v.values ++ [ { defaultExpr | ref = model.file.nextRef } ] } }
                      }
                    ]

                _ ->
                    []
           )
        ++ (contextActions ctx expr)


contextActions : Context -> Ast.Expression -> List Action
contextActions ctx expr =
    List.concatMap (refActions expr) <| List.map (\( r, n ) -> n) <| ctx


refActions : Ast.Expression -> Node -> List Action
refActions expr node =
    case node of
        Pat pat ->
            case pat.pvalue of
                Ast.LabelValue l ->
                    [ { label = l.name
                      , msg = SetNode 1 <| Expr { expr | value = Ast.RefValue { ref = pat.ref } }
                      }
                    ]

                _ ->
                    []

        VarDef def ->
            case def.label of
                Just l ->
                    [ { label = l.name
                      , msg = SetNode 1 <| Expr { expr | value = Ast.RefValue { ref = def.ref } }
                      }
                    ]

                _ ->
                    []

        _ ->
            []


variableDefinitionActions : Model -> Ast.VariableDefinition -> List Action
variableDefinitionActions model def =
    [ { label = "set name"
      , msg = SetNode 0 <| VarDef { def | label = Just { name = model.input } }
      }
    , { label = "arg"
      , msg =
            SetNode 1 <|
                VarDef
                    { def
                        | arguments =
                            def.arguments
                                ++ [ { ref = model.file.nextRef
                                     , pvalue = Ast.LabelValue { name = "xyz" }
                                     }
                                   ]
                    }
      }
    ]


patternActions : Model -> Ast.Pattern -> List Action
patternActions model pat =
    case pat.pvalue of
        Ast.LabelValue v ->
            [ { label = "set name"
              , msg = SetNode 0 <| Pat { pat | pvalue = Ast.LabelValue { v | name = model.input } }
              }
            ]

        _ ->
            []


intActions : Model -> Ast.Expression -> List Action
intActions model expr =
    case String.toInt (model.input) of
        Ok n ->
            [ { label = (toString n) ++ " (Int)"
              , msg = SetNode 0 <| Expr { expr | value = Ast.IntValue { value = n } }
              }
            ]

        _ ->
            []


floatActions : Model -> Ast.Expression -> List Action
floatActions model expr =
    case String.toFloat (model.input) of
        Ok n ->
            [ { label = (toString n) ++ " (Float)"
              , msg = SetNode 0 <| Expr { expr | value = Ast.FloatValue { value = n } }
              }
            ]

        _ ->
            []
