module Ast exposing (..)


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


requiredFieldDecoder : String -> a -> JD.Decoder a -> JD.Decoder a
requiredFieldDecoder name default decoder =
  withDefault default (name := decoder)


optionalFieldDecoder : String -> JD.Decoder a -> JD.Decoder (Maybe a)
optionalFieldDecoder name decoder =
  optionalDecoder (name := decoder)


repeatedFieldDecoder : String -> JD.Decoder a -> JD.Decoder (List a)
repeatedFieldDecoder name decoder =
  withDefault [] (name := (JD.list decoder))


withDefault : a -> JD.Decoder a -> JD.Decoder a
withDefault default decoder =
  JD.oneOf
    [ decoder
    , JD.succeed default
    ]


optionalEncoder : String -> (a -> JE.Value) -> Maybe a -> Maybe (String, JE.Value)
optionalEncoder name encoder v =
  case v of
    Just x ->
      Just (name, encoder x)
    
    Nothing ->
      Nothing


requiredFieldEncoder : String -> (a -> JE.Value) -> a -> a -> Maybe (String, JE.Value)
requiredFieldEncoder name encoder default v =
  if
    v == default
  then
    Nothing
  else
    Just (name, encoder v)


repeatedFieldEncoder : String -> (a -> JE.Value) -> List a -> Maybe (String, JE.Value)
repeatedFieldEncoder name encoder v =
  case v of
    [] ->
      Nothing
    _ ->
      Just (name, JE.list <| List.map encoder v)


type alias Node =
  { name : String -- 1
  , expression : Maybe Expression -- 2
  }


nodeDecoder : JD.Decoder Node
nodeDecoder =
  Node
    <$> (requiredFieldDecoder "name" "" JD.string)
    <*> (optionalFieldDecoder "expression" expressionDecoder)


nodeEncoder : Node -> JE.Value
nodeEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "name" JE.string "" v.name)
    , (optionalEncoder "expression" expressionEncoder v.expression)
    ]


type alias Expression =
  { ref : Int -- 1
  , value : Value
  }


type Value
  = ValueUnspecified
  | BoolValue Expression_Bool
  | IntValue Expression_Int
  | FloatValue Expression_Float
  | StringValue Expression_String
  | ListValue Expression_List


valueDecoder : JD.Decoder Value
valueDecoder =
  JD.oneOf
    [ JD.map BoolValue ("boolValue" := expression_BoolDecoder)
    , JD.map IntValue ("intValue" := expression_IntDecoder)
    , JD.map FloatValue ("floatValue" := expression_FloatDecoder)
    , JD.map StringValue ("stringValue" := expression_StringDecoder)
    , JD.map ListValue ("listValue" := expression_ListDecoder)
    , JD.succeed ValueUnspecified
    ]


valueEncoder : Value -> Maybe (String, JE.Value)
valueEncoder v =
  case v of
    ValueUnspecified -> Nothing
    BoolValue x -> Just ("boolValue", expression_BoolEncoder x)
    IntValue x -> Just ("intValue", expression_IntEncoder x)
    FloatValue x -> Just ("floatValue", expression_FloatEncoder x)
    StringValue x -> Just ("stringValue", expression_StringEncoder x)
    ListValue x -> Just ("listValue", expression_ListEncoder x)


expressionDecoder : JD.Decoder Expression
expressionDecoder =
  Expression
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> valueDecoder


expressionEncoder : Expression -> JE.Value
expressionEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (valueEncoder v.value)
    ]


type alias Expression_Bool =
  { value : Bool -- 1
  }


expression_BoolDecoder : JD.Decoder Expression_Bool
expression_BoolDecoder =
  Expression_Bool
    <$> (requiredFieldDecoder "value" False JD.bool)


expression_BoolEncoder : Expression_Bool -> JE.Value
expression_BoolEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "value" JE.bool False v.value)
    ]


type alias Expression_Int =
  { value : Int -- 1
  }


expression_IntDecoder : JD.Decoder Expression_Int
expression_IntDecoder =
  Expression_Int
    <$> (requiredFieldDecoder "value" 0 JD.int)


expression_IntEncoder : Expression_Int -> JE.Value
expression_IntEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "value" JE.int 0 v.value)
    ]


type alias Expression_Float =
  { value : Float -- 1
  }


expression_FloatDecoder : JD.Decoder Expression_Float
expression_FloatDecoder =
  Expression_Float
    <$> (requiredFieldDecoder "value" 0.0 JD.float)


expression_FloatEncoder : Expression_Float -> JE.Value
expression_FloatEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "value" JE.float 0.0 v.value)
    ]


type alias Expression_String =
  { value : String -- 1
  }


expression_StringDecoder : JD.Decoder Expression_String
expression_StringDecoder =
  Expression_String
    <$> (requiredFieldDecoder "value" "" JD.string)


expression_StringEncoder : Expression_String -> JE.Value
expression_StringEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "value" JE.string "" v.value)
    ]


type alias Expression_List =
  { values : List Expression -- 1
  }


expression_ListDecoder : JD.Decoder Expression_List
expression_ListDecoder =
  Expression_List
    <$> (repeatedFieldDecoder "values" expressionDecoder)


expression_ListEncoder : Expression_List -> JE.Value
expression_ListEncoder v =
  JE.object <| List.filterMap identity <|
    [ (repeatedFieldEncoder "values" expressionEncoder v.values)
    ]


type alias Expression_If =
  { cond : Maybe Expression -- 1
  , true : Maybe Expression -- 2
  , false : Maybe Expression -- 3
  }


expression_IfDecoder : JD.Decoder Expression_If
expression_IfDecoder =
  Expression_If
    <$> (optionalFieldDecoder "cond" expressionDecoder)
    <*> (optionalFieldDecoder "true" expressionDecoder)
    <*> (optionalFieldDecoder "false" expressionDecoder)


expression_IfEncoder : Expression_If -> JE.Value
expression_IfEncoder v =
  JE.object <| List.filterMap identity <|
    [ (optionalEncoder "cond" expressionEncoder v.cond)
    , (optionalEncoder "true" expressionEncoder v.true)
    , (optionalEncoder "false" expressionEncoder v.false)
    ]
