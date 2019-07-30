module Actions exposing (argumentActions, contextActions, expressionActions, floatActions, functionDefinitionActions, intActions, nodeActions, refActions)

import Defaults exposing (..)
import GetNode exposing (..)
import Html exposing (..)
import Html.Events exposing (..)
import List.Extra
import Proto.Ast as Ast
import String
import Types exposing (..)


nodeActions : Model -> Node -> Context -> List Action
nodeActions model node ctx =
    let
        movementActions =
            [ { label = "↑"
              , msg = MoveOut
              }
            , { label = "↓"
              , msg = MoveIn
              }
            , { label = "←"
              , msg = MoveLeft
              }
            , { label = "→"
              , msg = MoveRight
              }
            , { label = "Create function"
              , msg = CreateFunction
              }
            ]

        actions =
            case node of
                Expr expr ->
                    expressionActions model ctx expr

                FuncDef fdef ->
                    functionDefinitionActions model fdef

                Arg arg ->
                    argumentActions model arg
    in
    movementActions ++ actions


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
    , { label = "fn"
      , msg =
            SetNode 2 <|
                Expr
                    { expr
                        | value =
                            Ast.FunctionDefinition
                                { ref = model.file.nextRef
                                , label = Nothing
                                , arguments = []
                                , returnType = Nothing
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
                        Ast.FunctionApplicationValue
                            { function = 1
                            , arguments = []
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
                        Ast.FunctionApplicationValue
                            { function = 1
                            , arguments = []
                            }
                    }
      }
    , { label = "◆"
      , msg = SetNode 0 <| Expr defaultExpr
      }
    , { label = "⌧"
      , msg = DeleteNode
      }
    , { label = "\"" ++ model.input ++ "\" (String)"
      , msg = SetNode 0 <| Expr { expr | value = Ast.StringValue { value = model.input } }
      }
    ]
        ++ (case String.toInt model.input of
                Just v ->
                    [ { label = String.fromInt v ++ " (Int)"
                      , msg = SetNode 0 <| Expr { expr | value = Ast.IntValue { value = v } }
                      }
                    ]

                Nothing ->
                    []
           )
        ++ (case String.toFloat model.input of
                Just v ->
                    [ { label = String.fromFloat v ++ " (Float)"
                      , msg = SetNode 0 <| Expr { expr | value = Ast.FloatValue { value = v } }
                      }
                    ]

                Nothing ->
                    []
           )
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
        ++ contextActions ctx expr


contextActions : Context -> Ast.Expression -> List Action
contextActions ctx expr =
    List.concatMap (refActions expr) <| List.map (\( r, n ) -> n) <| ctx


refActions : Ast.Expression -> Node -> List Action
refActions expr node =
    case node of
        Expr _ ->
            []

        FuncDef def ->
            case def.label of
                Just l ->
                    [ { label = l.name
                      , msg = SetNode 1 <| Expr { expr | value = Ast.RefValue { ref = def.ref } }
                      }
                    ]

                Nothing ->
                    []

        Arg arg ->
            case arg.label of
                Just l ->
                    [ { label = l.name
                      , msg = SetNode 1 <| Expr { expr | value = Ast.RefValue { ref = arg.ref } }
                      }
                    ]

                Nothing ->
                    []


functionDefinitionActions : Model -> Ast.FunctionDefinition -> List Action
functionDefinitionActions model def =
    [ { label = "set name"
      , msg =
            SetNode 0 <|
                FuncDef
                    { def
                        | label = Just { name = model.input, colour = "black" }
                    }
      }
    , { label = "arg"
      , msg =
            SetNode 1 <|
                FuncDef
                    { def
                        | arguments =
                            def.arguments
                                ++ [ { ref = model.file.nextRef
                                     , label = Just { name = "xyz", colour = "black" }
                                     , type_ = Nothing
                                     }
                                   ]
                    }
      }
    ]


argumentActions : Model -> Ast.Argument -> List Action
argumentActions model arg =
    case arg.pvalue of
        Ast.LabelValue v ->
            [ { label = "set name"
              , msg = SetNode 0 <| Arg { arg | label = Ast.LabelValue { v | name = model.input } }
              }
            ]

        _ ->
            []


intActions : Model -> Ast.Expression -> List Action
intActions model expr =
    case String.toInt model.input of
        Just n ->
            [ { label = String.fromInt n ++ " (Int)"
              , msg = SetNode 0 <| Expr { expr | value = Ast.IntValue { value = n } }
              }
            ]

        Nothing ->
            []


floatActions : Model -> Ast.Expression -> List Action
floatActions model expr =
    case String.toFloat model.input of
        Just n ->
            [ { label = String.fromFloat n ++ " (Float)"
              , msg = SetNode 0 <| Expr { expr | value = Ast.FloatValue { value = n } }
              }
            ]

        Nothing ->
            []
