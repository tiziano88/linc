use crate::schema::*;

const INDENT: &str = "  ";
const UNKNOWN: &str = "unknown";

pub fn pretty_print(value: &FieldValue, schema: &Schema) -> String {
    let unknown = UNKNOWN.to_owned();
    match value {
        FieldValue::String(s) => format!("\"{}\"", s),
        FieldValue::Bytes(b) => format!("\"{}\"", String::from_utf8_lossy(b)),
        FieldValue::Bool(b) => format!("{}", b),
        FieldValue::Int(i) => format!("{}", i),
        FieldValue::Float(f) => format!("{}", f),
        FieldValue::Object(o) => {
            let mut s = String::new();
            let kind_name = schema
                .get_kind(o.kind_id)
                .map(|k| &k.name)
                .unwrap_or(&unknown);
            s.push_str(&kind_name);
            s.push_str(" {\n");
            for (i, (field_id, value)) in o.fields.iter().enumerate() {
                let mut f = String::new();
                let field_name = schema
                    .get_kind(o.kind_id)
                    .and_then(|k| k.get_field(*field_id))
                    .map(|f| &f.name)
                    .unwrap_or(&unknown);
                f.push_str(field_name);
                f.push_str(": ");
                f.push_str(&pretty_print(value, schema));
                s.push_str(&indent(&f, 1));
            }
            s.push_str("}");
            s
        }
    }
}

fn indent(s: &str, n: u32) -> String {
    let mut out = String::new();
    for line in s.lines() {
        for _ in 0..n {
            out.push_str(INDENT);
        }
        out.push_str(line);
        out.push_str("\n");
    }
    out
}
