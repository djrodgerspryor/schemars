use crate::SchemaGenerator;
use crate::{json_schema, JsonSchema, Schema};
use alloc::borrow::Cow;

impl JsonSchema for http::Uri {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "Uri".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "http::Uri".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "format": "uri",
        })
    }
}

impl JsonSchema for http::Method {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "Method".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "http::Method".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
          "enum": ["GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "CONNECT", "PATCH", "TRACE"]
        })
    }
}
