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


type alias Expression =
  { ref : Int -- 1
  , name : String -- 111
  , value : Value
  , type1 : Type1
  }


type Value
  = ValueUnspecified
  | EmptyValue Int
  | BoolValue Expression_Bool
  | IntValue Expression_Int
  | FloatValue Expression_Float
  | StringValue Expression_String
  | ListValue Expression_List
  | IfValue Expression_If


valueDecoder : JD.Decoder Value
valueDecoder =
  JD.oneOf
    [ JD.map EmptyValue ("emptyValue" := JD.int)
    , JD.map BoolValue ("boolValue" := expression_BoolDecoder)
    , JD.map IntValue ("intValue" := expression_IntDecoder)
    , JD.map FloatValue ("floatValue" := expression_FloatDecoder)
    , JD.map StringValue ("stringValue" := expression_StringDecoder)
    , JD.map ListValue ("listValue" := expression_ListDecoder)
    , JD.map IfValue ("ifValue" := expression_IfDecoder)
    , JD.succeed ValueUnspecified
    ]


valueEncoder : Value -> Maybe (String, JE.Value)
valueEncoder v =
  case v of
    ValueUnspecified -> Nothing
    EmptyValue x -> Just ("emptyValue", JE.int x)
    BoolValue x -> Just ("boolValue", expression_BoolEncoder x)
    IntValue x -> Just ("intValue", expression_IntEncoder x)
    FloatValue x -> Just ("floatValue", expression_FloatEncoder x)
    StringValue x -> Just ("stringValue", expression_StringEncoder x)
    ListValue x -> Just ("listValue", expression_ListEncoder x)
    IfValue x -> Just ("ifValue", expression_IfEncoder x)


type Type1
  = Type1Unspecified
  | Xxx Int


type1Decoder : JD.Decoder Type1
type1Decoder =
  JD.oneOf
    [ JD.map Xxx ("xxx" := JD.int)
    , JD.succeed Type1Unspecified
    ]


type1Encoder : Type1 -> Maybe (String, JE.Value)
type1Encoder v =
  case v of
    Type1Unspecified -> Nothing
    Xxx x -> Just ("xxx", JE.int x)


expressionDecoder : JD.Decoder Expression
expressionDecoder =
  Expression
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> (requiredFieldDecoder "name" "" JD.string)
    <*> valueDecoder
    <*> type1Decoder


expressionEncoder : Expression -> JE.Value
expressionEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (requiredFieldEncoder "name" JE.string "" v.name)
    , (valueEncoder v.value)
    , (type1Encoder v.type1)
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
