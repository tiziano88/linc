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


type alias File =
  { nextRef : Int -- 1
  , name : String -- 2
  , context : List Expression -- 3
  , typeAliases : List TypeAlias -- 4
  }


fileDecoder : JD.Decoder File
fileDecoder =
  File
    <$> (requiredFieldDecoder "nextRef" 0 JD.int)
    <*> (requiredFieldDecoder "name" "" JD.string)
    <*> (repeatedFieldDecoder "context" expressionDecoder)
    <*> (repeatedFieldDecoder "typeAliases" typeAliasDecoder)


fileEncoder : File -> JE.Value
fileEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "nextRef" JE.int 0 v.nextRef)
    , (requiredFieldEncoder "name" JE.string "" v.name)
    , (repeatedFieldEncoder "context" expressionEncoder v.context)
    , (repeatedFieldEncoder "typeAliases" typeAliasEncoder v.typeAliases)
    ]


type alias Expression =
  { ref : Int -- 1
  , name : String -- 111
  , type1 : Maybe Type -- 89
  , value : Value
  , arguments : Arguments
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
  | LambdaValue Expression_Lambda
  | RefValue Expression_Ref


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
    , JD.map LambdaValue ("lambdaValue" := expression_LambdaDecoder)
    , JD.map RefValue ("refValue" := expression_RefDecoder)
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
    LambdaValue x -> Just ("lambdaValue", expression_LambdaEncoder x)
    RefValue x -> Just ("refValue", expression_RefEncoder x)


type Arguments
  = ArgumentsUnspecified
  | Args Expression_Arguments


argumentsDecoder : JD.Decoder Arguments
argumentsDecoder =
  JD.oneOf
    [ JD.map Args ("args" := expression_ArgumentsDecoder)
    , JD.succeed ArgumentsUnspecified
    ]


argumentsEncoder : Arguments -> Maybe (String, JE.Value)
argumentsEncoder v =
  case v of
    ArgumentsUnspecified -> Nothing
    Args x -> Just ("args", expression_ArgumentsEncoder x)


expressionDecoder : JD.Decoder Expression
expressionDecoder =
  Expression
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> (requiredFieldDecoder "name" "" JD.string)
    <*> (optionalFieldDecoder "type1" typeDecoder)
    <*> valueDecoder
    <*> argumentsDecoder


expressionEncoder : Expression -> JE.Value
expressionEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (requiredFieldEncoder "name" JE.string "" v.name)
    , (optionalEncoder "type1" typeEncoder v.type1)
    , (valueEncoder v.value)
    , (argumentsEncoder v.arguments)
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


type alias Expression_Lambda =
  { argument : Maybe Expression -- 1
  , body : Maybe Expression -- 2
  }


expression_LambdaDecoder : JD.Decoder Expression_Lambda
expression_LambdaDecoder =
  Expression_Lambda
    <$> (optionalFieldDecoder "argument" expressionDecoder)
    <*> (optionalFieldDecoder "body" expressionDecoder)


expression_LambdaEncoder : Expression_Lambda -> JE.Value
expression_LambdaEncoder v =
  JE.object <| List.filterMap identity <|
    [ (optionalEncoder "argument" expressionEncoder v.argument)
    , (optionalEncoder "body" expressionEncoder v.body)
    ]


type alias Expression_Ref =
  { ref : Int -- 1
  }


expression_RefDecoder : JD.Decoder Expression_Ref
expression_RefDecoder =
  Expression_Ref
    <$> (requiredFieldDecoder "ref" 0 JD.int)


expression_RefEncoder : Expression_Ref -> JE.Value
expression_RefEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    ]


type alias Expression_Arguments =
  { values : List Expression -- 1
  }


expression_ArgumentsDecoder : JD.Decoder Expression_Arguments
expression_ArgumentsDecoder =
  Expression_Arguments
    <$> (repeatedFieldDecoder "values" expressionDecoder)


expression_ArgumentsEncoder : Expression_Arguments -> JE.Value
expression_ArgumentsEncoder v =
  JE.object <| List.filterMap identity <|
    [ (repeatedFieldEncoder "values" expressionEncoder v.values)
    ]


type alias VariableDefinition =
  { ref : Int -- 1
  , name : String -- 2
  , value : Maybe Expression -- 3
  }


variableDefinitionDecoder : JD.Decoder VariableDefinition
variableDefinitionDecoder =
  VariableDefinition
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> (requiredFieldDecoder "name" "" JD.string)
    <*> (optionalFieldDecoder "value" expressionDecoder)


variableDefinitionEncoder : VariableDefinition -> JE.Value
variableDefinitionEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (requiredFieldEncoder "name" JE.string "" v.name)
    , (optionalEncoder "value" expressionEncoder v.value)
    ]


type alias TypeAlias =
  { ref : Int -- 1
  , name : String -- 10
  , type1 : Maybe Type -- 20
  }


typeAliasDecoder : JD.Decoder TypeAlias
typeAliasDecoder =
  TypeAlias
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> (requiredFieldDecoder "name" "" JD.string)
    <*> (optionalFieldDecoder "type1" typeDecoder)


typeAliasEncoder : TypeAlias -> JE.Value
typeAliasEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (requiredFieldEncoder "name" JE.string "" v.name)
    , (optionalEncoder "type1" typeEncoder v.type1)
    ]


type alias Symbol =
  { ref : Int -- 1
  , svalue : Svalue
  }


type Svalue
  = SvalueUnspecified
  | Constructor String
  | Variable String
  | Placeholder Bool


svalueDecoder : JD.Decoder Svalue
svalueDecoder =
  JD.oneOf
    [ JD.map Constructor ("constructor" := JD.string)
    , JD.map Variable ("variable" := JD.string)
    , JD.map Placeholder ("placeholder" := JD.bool)
    , JD.succeed SvalueUnspecified
    ]


svalueEncoder : Svalue -> Maybe (String, JE.Value)
svalueEncoder v =
  case v of
    SvalueUnspecified -> Nothing
    Constructor x -> Just ("constructor", JE.string x)
    Variable x -> Just ("variable", JE.string x)
    Placeholder x -> Just ("placeholder", JE.bool x)


symbolDecoder : JD.Decoder Symbol
symbolDecoder =
  Symbol
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> svalueDecoder


symbolEncoder : Symbol -> JE.Value
symbolEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (svalueEncoder v.svalue)
    ]


type alias Type =
  { ref : Int -- 1
  , tvalue : Tvalue
  }


type Tvalue
  = TvalueUnspecified
  | Primitive Type_PrimitiveType
  | Compound Type_CompoundType
  | RefType Type_RefType


tvalueDecoder : JD.Decoder Tvalue
tvalueDecoder =
  JD.oneOf
    [ JD.map Primitive ("primitive" := type_PrimitiveTypeDecoder)
    , JD.map Compound ("compound" := type_CompoundTypeDecoder)
    , JD.map RefType ("refType" := type_RefTypeDecoder)
    , JD.succeed TvalueUnspecified
    ]


tvalueEncoder : Tvalue -> Maybe (String, JE.Value)
tvalueEncoder v =
  case v of
    TvalueUnspecified -> Nothing
    Primitive x -> Just ("primitive", type_PrimitiveTypeEncoder x)
    Compound x -> Just ("compound", type_CompoundTypeEncoder x)
    RefType x -> Just ("refType", type_RefTypeEncoder x)


type Type_PrimitiveType
  = Type_PrimitiveTypeUnspecified -- 0
  | Type_Int -- 1
  | Type_Float -- 2


typeDecoder : JD.Decoder Type
typeDecoder =
  Type
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> tvalueDecoder


type_PrimitiveTypeDecoder : JD.Decoder Type_PrimitiveType
type_PrimitiveTypeDecoder =
  let
    lookup s = case s of
      "PRIMITIVE_TYPE_UNSPECIFIED" -> Type_PrimitiveTypeUnspecified
      "INT" -> Type_Int
      "FLOAT" -> Type_Float
      _ -> Type_PrimitiveTypeUnspecified
  in
    JD.map lookup JD.string


type_PrimitiveTypeDefault : Type_PrimitiveType
type_PrimitiveTypeDefault = Type_PrimitiveTypeUnspecified


typeEncoder : Type -> JE.Value
typeEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (tvalueEncoder v.tvalue)
    ]


type_PrimitiveTypeEncoder : Type_PrimitiveType -> JE.Value
type_PrimitiveTypeEncoder v =
  let
    lookup s = case s of
      Type_PrimitiveTypeUnspecified -> "PRIMITIVE_TYPE_UNSPECIFIED"
      Type_Int -> "INT"
      Type_Float -> "FLOAT"
  in
    JE.string <| lookup v


type alias Type_RefType =
  { ref : Int -- 1
  }


type_RefTypeDecoder : JD.Decoder Type_RefType
type_RefTypeDecoder =
  Type_RefType
    <$> (requiredFieldDecoder "ref" 0 JD.int)


type_RefTypeEncoder : Type_RefType -> JE.Value
type_RefTypeEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    ]


type alias Type_OpaqueType =
  { name : String -- 1
  }


type_OpaqueTypeDecoder : JD.Decoder Type_OpaqueType
type_OpaqueTypeDecoder =
  Type_OpaqueType
    <$> (requiredFieldDecoder "name" "" JD.string)


type_OpaqueTypeEncoder : Type_OpaqueType -> JE.Value
type_OpaqueTypeEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "name" JE.string "" v.name)
    ]


type alias Type_CompoundType =
  { x : Maybe Type -- 1
  , y : Maybe Type -- 2
  }


type_CompoundTypeDecoder : JD.Decoder Type_CompoundType
type_CompoundTypeDecoder =
  Type_CompoundType
    <$> (optionalFieldDecoder "x" typeDecoder)
    <*> (optionalFieldDecoder "y" typeDecoder)


type_CompoundTypeEncoder : Type_CompoundType -> JE.Value
type_CompoundTypeEncoder v =
  JE.object <| List.filterMap identity <|
    [ (optionalEncoder "x" typeEncoder v.x)
    , (optionalEncoder "y" typeEncoder v.y)
    ]


type alias TypeConstructor =
  { ref : Int -- 1
  , name : String -- 2
  }


typeConstructorDecoder : JD.Decoder TypeConstructor
typeConstructorDecoder =
  TypeConstructor
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> (requiredFieldDecoder "name" "" JD.string)


typeConstructorEncoder : TypeConstructor -> JE.Value
typeConstructorEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (requiredFieldEncoder "name" JE.string "" v.name)
    ]


type alias Pattern =
  { ref : Int -- 1
  , vvv : Vvv
  }


type Vvv
  = VvvUnspecified
  | TypeConstructorValue TypeConstructor
  | SymbolValue Symbol
  | PatternValue Pattern


vvvDecoder : JD.Decoder Vvv
vvvDecoder =
  JD.oneOf
    [ JD.map TypeConstructorValue ("typeConstructorValue" := typeConstructorDecoder)
    , JD.map SymbolValue ("symbolValue" := symbolDecoder)
    , JD.map PatternValue ("patternValue" := patternDecoder)
    , JD.succeed VvvUnspecified
    ]


vvvEncoder : Vvv -> Maybe (String, JE.Value)
vvvEncoder v =
  case v of
    VvvUnspecified -> Nothing
    TypeConstructorValue x -> Just ("typeConstructorValue", typeConstructorEncoder x)
    SymbolValue x -> Just ("symbolValue", symbolEncoder x)
    PatternValue x -> Just ("patternValue", patternEncoder x)


patternDecoder : JD.Decoder Pattern
patternDecoder =
  Pattern
    <$> (requiredFieldDecoder "ref" 0 JD.int)
    <*> vvvDecoder


patternEncoder : Pattern -> JE.Value
patternEncoder v =
  JE.object <| List.filterMap identity <|
    [ (requiredFieldEncoder "ref" JE.int 0 v.ref)
    , (vvvEncoder v.vvv)
    ]
