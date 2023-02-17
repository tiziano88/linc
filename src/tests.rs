use crate::{pretty_print::*, schema::*};

fn schema() -> Schema {
    Schema {
        kinds: vec![
            Kind {
                kind_id: 1,
                name: "root".to_string(),
                fields: vec![
                    Field {
                        field_id: 1,
                        name: "hello".to_string(),
                        type_: FieldType::String,
                    },
                    Field {
                        field_id: 2,
                        name: "world".to_string(),
                        type_: FieldType::String,
                    },
                    Field {
                        field_id: 3,
                        name: "country".to_string(),
                        type_: FieldType::Object { kind_id: 2 },
                    },
                ],
            },
            Kind {
                kind_id: 2,
                name: "country".to_string(),
                fields: vec![
                    Field {
                        field_id: 1,
                        name: "size".to_string(),
                        type_: FieldType::String,
                    },
                    Field {
                        field_id: 2,
                        name: "population".to_string(),
                        type_: FieldType::Int,
                    },
                    Field {
                        field_id: 4,
                        name: "friends_with".to_string(),
                        type_: FieldType::Object { kind_id: 2 },
                    },
                    Field {
                        field_id: 3,
                        name: "name".to_string(),
                        type_: FieldType::String,
                    },
                ],
            },
        ],
    }
}

#[test]
fn test_format_empty() {
    let out = pretty_print(
        &FieldValue::Object(Object {
            kind_id: 1,
            fields: vec![],
        }),
        &schema(),
    );
    assert_eq!(
        out,
        r#"root {
}"#
    )
}

#[test]
fn test_format_simple() {
    let out = pretty_print(
        &FieldValue::Object(Object {
            kind_id: 1,
            fields: vec![
                (1, FieldValue::String("hello_val".to_string())),
                (2, FieldValue::String("world_val".to_string())),
            ],
        }),
        &schema(),
    );
    assert_eq!(
        out,
        r#"root {
  hello: "hello_val"
  world: "world_val"
}"#
    )
}

#[test]
fn test_format_nested() {
    let out = pretty_print(
        &FieldValue::Object(Object {
            kind_id: 1,
            fields: vec![
                (1, FieldValue::String("hello_val".to_string())),
                (2, FieldValue::String("world_val".to_string())),
                (
                    3,
                    FieldValue::Object(Object {
                        kind_id: 2,
                        fields: vec![
                            (3, FieldValue::String("italy".to_string())),
                            (1, FieldValue::String("big".to_string())),
                            (2, FieldValue::Int(1000000000)),
                        ],
                    }),
                ),
            ],
        }),
        &schema(),
    );
    assert_eq!(
        out,
        r#"root {
  hello: "hello_val"
  world: "world_val"
  country: country {
    name: "italy"
    size: "big"
    population: 1000000000
  }
}"#
    )
}

#[test]
fn test_format_recursive() {
    let out = pretty_print(
        &FieldValue::Object(Object {
            kind_id: 1,
            fields: vec![
                (1, FieldValue::String("hello_val".to_string())),
                (2, FieldValue::String("world_val".to_string())),
                (
                    3,
                    FieldValue::Object(Object {
                        kind_id: 2,
                        fields: vec![
                            (3, FieldValue::String("italy".to_string())),
                            (1, FieldValue::String("big".to_string())),
                            (2, FieldValue::Int(1000000000)),
                            (
                                4,
                                FieldValue::Object(Object {
                                    kind_id: 2,
                                    fields: vec![
                                        (3, FieldValue::String("france".to_string())),
                                        (1, FieldValue::String("small".to_string())),
                                        (2, FieldValue::Int(100000)),
                                    ],
                                }),
                            ),
                        ],
                    }),
                ),
            ],
        }),
        &schema(),
    );
    assert_eq!(
        out,
        r#"root {
  hello: "hello_val"
  world: "world_val"
  country: country {
    name: "italy"
    size: "big"
    population: 1000000000
    friends_with: country {
      name: "france"
      size: "small"
      population: 100000
    }
  }
}"#
    )
}
