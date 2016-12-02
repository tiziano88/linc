module Proto.Server exposing (..)


import Json.Decode as JD
import Json.Encode as JE


(<$>) : (a -> b) -> JD.Decoder a -> JD.Decoder b
(<$>) =
    JD.map


(<*>) : JD.Decoder (a -> b) -> JD.Decoder a -> JD.Decoder b
(<*>) f v =
    f |> JD.andThen (\x -> x <$> v)


optionalDecoder : JD.Decoder a -> JD.Decoder (Maybe a)
optionalDecoder decoder =
    JD.oneOf
        [ JD.map Just decoder
        , JD.succeed Nothing
        ]


requiredFieldDecoder : String -> a -> JD.Decoder a -> JD.Decoder a
requiredFieldDecoder name default decoder =
    withDefault default (JD.field name decoder)


optionalFieldDecoder : String -> JD.Decoder a -> JD.Decoder (Maybe a)
optionalFieldDecoder name decoder =
    optionalDecoder (JD.field name decoder)


repeatedFieldDecoder : String -> JD.Decoder a -> JD.Decoder (List a)
repeatedFieldDecoder name decoder =
    withDefault [] (JD.field name (JD.list decoder))


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
            Just ( name, encoder x )

        Nothing ->
            Nothing


requiredFieldEncoder : String -> (a -> JE.Value) -> a -> a -> Maybe ( String, JE.Value )
requiredFieldEncoder name encoder default v =
    if v == default then
        Nothing
    else
        Just ( name, encoder v )


repeatedFieldEncoder : String -> (a -> JE.Value) -> List a -> Maybe (String, JE.Value)
repeatedFieldEncoder name encoder v =
    case v of
        [] ->
            Nothing
        _ ->
            Just (name, JE.list <| List.map encoder v)


type alias GetFileRequest =
    { path : String -- 1
    }


getFileRequestDecoder : JD.Decoder GetFileRequest
getFileRequestDecoder =
    JD.lazy <| \_ -> GetFileRequest
        <$> (requiredFieldDecoder "path" "" JD.string)


getFileRequestEncoder : GetFileRequest -> JE.Value
getFileRequestEncoder v =
    JE.object <| List.filterMap identity <|
        [ (requiredFieldEncoder "path" JE.string "" v.path)
        ]


type alias GetFileResponse =
    { jsonContent : String -- 1
    }


getFileResponseDecoder : JD.Decoder GetFileResponse
getFileResponseDecoder =
    JD.lazy <| \_ -> GetFileResponse
        <$> (requiredFieldDecoder "jsonContent" "" JD.string)


getFileResponseEncoder : GetFileResponse -> JE.Value
getFileResponseEncoder v =
    JE.object <| List.filterMap identity <|
        [ (requiredFieldEncoder "jsonContent" JE.string "" v.jsonContent)
        ]


type alias UpdateFileRequest =
    { path : String -- 1
    , jsonContent : String -- 2
    , elmContent : String -- 3
    }


updateFileRequestDecoder : JD.Decoder UpdateFileRequest
updateFileRequestDecoder =
    JD.lazy <| \_ -> UpdateFileRequest
        <$> (requiredFieldDecoder "path" "" JD.string)
        <*> (requiredFieldDecoder "jsonContent" "" JD.string)
        <*> (requiredFieldDecoder "elmContent" "" JD.string)


updateFileRequestEncoder : UpdateFileRequest -> JE.Value
updateFileRequestEncoder v =
    JE.object <| List.filterMap identity <|
        [ (requiredFieldEncoder "path" JE.string "" v.path)
        , (requiredFieldEncoder "jsonContent" JE.string "" v.jsonContent)
        , (requiredFieldEncoder "elmContent" JE.string "" v.elmContent)
        ]
