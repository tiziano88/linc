module proto.Ast exposing (..)


import Json.Decode as JD exposing ((:=))
import Json.Encode as JE


(<$>) : (a -> b) -> JD.Decoder a -> JD.Decoder b
(<$>) =
  JD.map


(<*>) : JD.Decoder (a -> b) -> JD.Decoder a -> JD.Decoder b
(<*>) f v =
  f `JD.andThen` \x -> x <$> v


optionalDecoder : JD.Decoder a -> JD.Decoder (Maybe a)
optionalDecoder decoder =
  JD.oneOf
    [ JD.map Just decoder
    , JD.succeed Nothing
    ]


optionalFieldDecoder : JD.Decoder a -> String -> JD.Decoder (Maybe a)
optionalFieldDecoder decoder name =
  optionalDecoder (name := decoder)


repeatedFieldDecoder : JD.Decoder a -> JD.Decoder (List a)
repeatedFieldDecoder decoder =
  withDefault [] (JD.list decoder)


withDefault : a -> JD.Decoder a -> JD.Decoder a
withDefault default decoder =
  JD.oneOf
    [ decoder
    , JD.succeed default
    ]


intFieldDecoder : String -> JD.Decoder Int
intFieldDecoder name =
  withDefault 0 (name := JD.int)


floatFieldDecoder : String -> JD.Decoder Float
floatFieldDecoder name =
  withDefault 0.0 (name := JD.float)


boolFieldDecoder : String -> JD.Decoder Bool
boolFieldDecoder name =
  withDefault False (name := JD.bool)


stringFieldDecoder : String -> JD.Decoder String
stringFieldDecoder name =
  withDefault "" (name := JD.string)


enumFieldDecoder : JD.Decoder a -> String -> JD.Decoder a
enumFieldDecoder decoder name =
  (name := decoder)


optionalEncoder : (a -> JE.Value) -> Maybe a -> JE.Value
optionalEncoder encoder v =
  case v of
    Just x ->
      encoder x
    
    Nothing ->
      JE.null


repeatedFieldEncoder : (a -> JE.Value) -> List a -> JE.Value
repeatedFieldEncoder encoder v =
  JE.list <| List.map encoder v


type alias Node =
  { name : String -- 1
  , expression : Maybe Expression -- 2
  }


nodeDecoder : JD.Decoder Node
nodeDecoder =
  Node
    <$> (stringFieldDecoder "name")
    <*> (optionalFieldDecoder expressionDecoder "expression")


nodeEncoder : Node -> JE.Value
nodeEncoder v =
  JE.object
    [ ("name", JE.string v.name)
    , ("expression", optionalEncoder expressionEncoder v.expression)
    ]


type alias Expression =
  { ref : Int -- 1
  , bool : Maybe Expression_Bool -- 2
  , int : Maybe Expression_Int -- 3
  , float : Maybe Expression_Float -- 4
  , string : Maybe Expression_String -- 5
  , list : Maybe Expression_List -- 6
  , if : Maybe Expression_If -- 7
  }


expressionDecoder : JD.Decoder Expression
expressionDecoder =
  Expression
    <$> (intFieldDecoder "ref")
    <*> (optionalFieldDecoder expression_BoolDecoder "bool")
    <*> (optionalFieldDecoder expression_IntDecoder "int")
    <*> (optionalFieldDecoder expression_FloatDecoder "float")
    <*> (optionalFieldDecoder expression_StringDecoder "string")
    <*> (optionalFieldDecoder expression_ListDecoder "list")
    <*> (optionalFieldDecoder expression_IfDecoder "if")


expressionEncoder : Expression -> JE.Value
expressionEncoder v =
  JE.object
    [ ("ref", JE.int v.ref)
    , ("bool", optionalEncoder expression_BoolEncoder v.bool)
    , ("int", optionalEncoder expression_IntEncoder v.int)
    , ("float", optionalEncoder expression_FloatEncoder v.float)
    , ("string", optionalEncoder expression_StringEncoder v.string)
    , ("list", optionalEncoder expression_ListEncoder v.list)
    , ("if", optionalEncoder expression_IfEncoder v.if)
    ]


type alias Expression_Bool =
  { value : Bool -- 1
  }


expression_BoolDecoder : JD.Decoder Expression_Bool
expression_BoolDecoder =
  Expression_Bool
    <$> (boolFieldDecoder "value")


expression_BoolEncoder : Expression_Bool -> JE.Value
expression_BoolEncoder v =
  JE.object
    [ ("value", JE.bool v.value)
    ]


type alias Expression_Int =
  { value : Int -- 1
  }


expression_IntDecoder : JD.Decoder Expression_Int
expression_IntDecoder =
  Expression_Int
    <$> (intFieldDecoder "value")


expression_IntEncoder : Expression_Int -> JE.Value
expression_IntEncoder v =
  JE.object
    [ ("value", JE.int v.value)
    ]


type alias Expression_Float =
  { value : Float -- 1
  }


expression_FloatDecoder : JD.Decoder Expression_Float
expression_FloatDecoder =
  Expression_Float
    <$> (floatFieldDecoder "value")


expression_FloatEncoder : Expression_Float -> JE.Value
expression_FloatEncoder v =
  JE.object
    [ ("value", JE.float v.value)
    ]


type alias Expression_String =
  { value : String -- 1
  }


expression_StringDecoder : JD.Decoder Expression_String
expression_StringDecoder =
  Expression_String
    <$> (stringFieldDecoder "value")


expression_StringEncoder : Expression_String -> JE.Value
expression_StringEncoder v =
  JE.object
    [ ("value", JE.string v.value)
    ]


type alias Expression_List =
  { values : List Expression -- 1
  }


expression_ListDecoder : JD.Decoder Expression_List
expression_ListDecoder =
  Expression_List
    <$> (repeatedFieldDecoder (expressionDecoder "values"))


expression_ListEncoder : Expression_List -> JE.Value
expression_ListEncoder v =
  JE.object
    [ ("values", repeatedFieldEncoder expressionEncoder v.values)
    ]


type alias Expression_If =
  { cond : Maybe Expression -- 1
  , true : Maybe Expression -- 2
  , false : Maybe Expression -- 3
  }


expression_IfDecoder : JD.Decoder Expression_If
expression_IfDecoder =
  Expression_If
    <$> (optionalFieldDecoder expressionDecoder "cond")
    <*> (optionalFieldDecoder expressionDecoder "true")
    <*> (optionalFieldDecoder expressionDecoder "false")


expression_IfEncoder : Expression_If -> JE.Value
expression_IfEncoder v =
  JE.object
    [ ("cond", optionalEncoder expressionEncoder v.cond)
    , ("true", optionalEncoder expressionEncoder v.true)
    , ("false", optionalEncoder expressionEncoder v.false)
    ]


