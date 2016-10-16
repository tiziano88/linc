module Actions exposing (..)

import Html exposing (..)
import Html.Events exposing (..)
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

        b =
            [ { label = "↑"
              , msg = SetRefPath <| List.drop 1 model.refPath
              }
            , { label = "↓"
              , msg = Nop
              }
            , { label = "←"
              , msg = Nop
              }
            , { label = "→"
              , msg = Nop
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
                        , arguments = Ast.Args { values = [] }
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
                                            , arguments = expr.arguments
                                        }
                                , true = Just { defaultExpr | ref = model.file.nextRef + 1 }
                                , false = Just { defaultExpr | ref = model.file.nextRef + 2 }
                                }
                        , arguments = Ast.Args { values = [] }
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
                                , true = Just
                                        { defaultExpr
                                            | ref = model.file.nextRef + 1 
                                            , value = expr.value
                                            , arguments = expr.arguments
                                        }
                                , false = Just { defaultExpr | ref = model.file.nextRef + 2 }
                                }
                        , arguments = Ast.Args { values = [] }
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
                                , false = Just
                                        { defaultExpr
                                            | ref = model.file.nextRef + 2
                                            , value = expr.value
                                            , arguments = expr.arguments
                                        }
                                }
                        , arguments = Ast.Args { values = [] }
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
            SetNode 1 <|
                Expr
                    { expr
                        | arguments =
                            case expr.arguments of
                                Ast.Args a ->
                                    Ast.Args { values = a.values ++ [ { defaultExpr | ref = model.file.nextRef } ] }

                                Ast.ArgumentsUnspecified ->
                                    Ast.Args { values = [ { defaultExpr | ref = model.file.nextRef } ] }
                    }
      }
    , { label = "◆ ◇" -- TODO
      , msg =
            SetNode 1 <|
                Expr
                    { expr
                        | arguments =
                            case expr.arguments of
                                Ast.Args a ->
                                    Ast.Args { values = a.values ++ [ { defaultExpr | ref = model.file.nextRef } ] }

                                Ast.ArgumentsUnspecified ->
                                    Ast.Args { values = [ { defaultExpr | ref = model.file.nextRef } ] }
                    }
      }
    , { label = "◆"
      , msg = SetNode 0 <| Expr defaultExpr
      }
    , { label = "⌧" -- TODO: Delete node entirely.
      , msg = SetNode 0 <| Expr expr
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
