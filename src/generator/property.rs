use super::{BaseType, TypeRecordRef};
use crate::google::protobuf::field_options::{self};

#[derive(Clone)]
pub enum PropertyFormat {
    None,
    UnixTimeSeconds,
    UnixTimeMilliseconds,
    Guid,
    DateTime,
    DateTimeOffset,
    DateOnly,
    TimeOnly,
    TimeSpan,
}

impl PropertyFormat {
    pub fn from_proto(format_type: &field_options::FormatType) -> PropertyFormat {
        match format_type {
            field_options::FormatType::None => PropertyFormat::None,
            field_options::FormatType::UnixTimeSeconds => PropertyFormat::UnixTimeSeconds,
            field_options::FormatType::UnixTimeMilliseconds => PropertyFormat::UnixTimeMilliseconds,
            field_options::FormatType::Guid => PropertyFormat::Guid,
            field_options::FormatType::DateTime => PropertyFormat::DateTime,
            field_options::FormatType::DateTimeOffset => PropertyFormat::DateTimeOffset,
            field_options::FormatType::DateOnly => PropertyFormat::DateOnly,
            field_options::FormatType::TimeOnly => PropertyFormat::TimeOnly,
            field_options::FormatType::TimeSpan => PropertyFormat::TimeSpan,
        }
    }
}

#[derive(Clone)]
pub struct PropertyOptions {
    pub required: bool,
    pub eq: Option<f64>,
    pub ne: Option<f64>,
    pub lt: Option<f64>,
    pub lte: Option<f64>,
    pub gt: Option<f64>,
    pub gte: Option<f64>,
    pub len: Option<u64>,
    pub min_len: Option<u64>,
    pub max_len: Option<u64>,
    pub count: Option<u64>,
    pub min_count: Option<u64>,
    pub max_count: Option<u64>,
    pub key_format: PropertyFormat,
    pub format: PropertyFormat,
}

impl Default for PropertyOptions {
    fn default() -> Self {
        Self {
            required: false,
            eq: None,
            ne: None,
            lt: None,
            lte: None,
            gt: None,
            gte: None,
            len: None,
            min_len: None,
            max_len: None,
            count: None,
            min_count: None,
            max_count: None,
            key_format: PropertyFormat::None,
            format: PropertyFormat::None,
        }
    }
}

pub struct Property {
    type_record: TypeRecordRef,
    repeated: bool,
    name: String,
    options: PropertyOptions,
}

impl Property {
    pub fn new(type_record: TypeRecordRef, repeated: bool, name: &str, options: &PropertyOptions) -> Self {
        Property {
            type_record,
            repeated,
            name: name.to_string(),
            options: options.to_owned(),
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        self.type_record.namespace.as_deref()
    }

    pub fn base_type(&self) -> &BaseType {
        &self.type_record.base_type
    }

    pub fn type_name(&self, current_namespace: &str) -> String {
        if self.repeated() || matches!(self.base_type(), BaseType::Map(..)) {
            self.nested_type_without_parent(current_namespace)
        } else if let Some(parent_name) = &self.type_record.parent_name {
            format!("{}.{}", parent_name, self.nested_type_without_parent(current_namespace))
        } else {
            self.nested_type_without_parent(current_namespace)
        }
    }

    pub fn nested_type_without_parent(&self, current_namespace: &str) -> String {
        let mut type_name = match &self.base_type() {
            BaseType::Bool => "bool".to_string(),
            BaseType::Int => "int".to_string(),
            BaseType::Long => match self.options.format {
                PropertyFormat::UnixTimeSeconds => "System.DateTime".to_string(),
                PropertyFormat::UnixTimeMilliseconds => "System.DateTime".to_string(),
                _ => "long".to_string(),
            },
            BaseType::Uint => "uint".to_string(),
            BaseType::ULong => "ulong".to_string(),
            BaseType::Float => "float".to_string(),
            BaseType::Double => "double".to_string(),
            BaseType::Bytes => "Google.Protobuf.ByteString".to_string(),
            BaseType::String => match self.options.format {
                PropertyFormat::Guid => "System.Guid".to_string(),
                PropertyFormat::DateTime => "System.DateTime".to_string(),
                PropertyFormat::DateTimeOffset => "System.DateTimeOffset".to_string(),
                PropertyFormat::DateOnly => "System.DateOnly".to_string(),
                PropertyFormat::TimeOnly => "System.TimeOnly".to_string(),
                PropertyFormat::TimeSpan => "System.TimeSpan".to_string(),
                _ => "string".to_string(),
            },
            BaseType::Enum(x) => x.to_string(),
            BaseType::Message(x) => x.to_string(),
            BaseType::Map(k, v) => {
                let key_property = Property::new(k.clone(), false, "key", &self.options);
                let value_property = Property::new(v.clone(), false, "value", &self.options);
                format!(
                    "System.Collections.Generic.Dictionary<{}, {}>",
                    key_property.full_type_name(current_namespace),
                    value_property.full_type_name(current_namespace)
                )
            }
        };

        if self.nullable() {
            type_name = format!("{}?", type_name);
        }

        if self.repeated() {
            type_name = format!("System.Collections.Generic.List<{}>", type_name);
        }

        type_name
    }

    pub fn full_type_name(&self, current_namespace: &str) -> String {
        if self.repeated() || matches!(self.base_type(), BaseType::Map(..)) {
            self.type_name(current_namespace)
        } else if let Some(namespace) = self.namespace().take_if(|x| *x != current_namespace) {
            format!("{}.{}", namespace, self.type_name(current_namespace))
        } else {
            self.type_name(current_namespace)
        }
    }

    pub fn nullable(&self) -> bool {
        self.type_record.nullable && !self.options.required
    }

    pub fn repeated(&self) -> bool {
        self.repeated
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn default_value(&self) -> Option<&'static str> {
        if self.repeated() {
            return Some("[]");
        } else if matches!(self.base_type(), BaseType::Map(..)) {
            return Some("[]");
        } else if !self.nullable() && matches!(self.base_type(), BaseType::Bytes) {
            return Some("Google.Protobuf.ByteString.Empty");
        } else if !self.nullable() && matches!(self.base_type(), BaseType::String) && matches!(self.options.format, PropertyFormat::None) {
            return Some("string.Empty");
        } else {
            None
        }
    }

    pub fn options(&self) -> &PropertyOptions {
        &self.options
    }

    pub fn codec(&self) -> Option<String> {
        let nullable = self.nullable().then(|| "Nullable").unwrap_or_default();
        match (self.base_type(), &self.options().format) {
            (BaseType::Long, PropertyFormat::UnixTimeSeconds) => Some(format!(r#"{nullable}UnixTimeSecondsCodec"#)),
            (BaseType::Long, PropertyFormat::UnixTimeMilliseconds) => Some(format!(r#"{nullable}UnixTimeMillisecondsCodec"#)),
            (BaseType::String, PropertyFormat::Guid) => Some(format!(r#"{nullable}GuidCodec"#)),
            (BaseType::String, PropertyFormat::DateTime) => Some(format!(r#"{nullable}DateTimeCodec"#)),
            (BaseType::String, PropertyFormat::DateTimeOffset) => Some(format!(r#"{nullable}DateTimeOffsetCodec"#)),
            (BaseType::String, PropertyFormat::DateOnly) => Some(format!(r#"{nullable}DateOnlyCodec"#)),
            (BaseType::String, PropertyFormat::TimeOnly) => Some(format!(r#"{nullable}TimeOnlyCodec"#)),
            (BaseType::String, PropertyFormat::TimeSpan) => Some(format!(r#"{nullable}TimeSpanCodec"#)),
            _ => None,
        }
    }

    pub fn is_checked(&self) -> bool {
        match (self.base_type(), &self.options().format) {
            (BaseType::String, PropertyFormat::Guid)
            | (BaseType::String, PropertyFormat::DateTime)
            | (BaseType::String, PropertyFormat::DateTimeOffset)
            | (BaseType::String, PropertyFormat::DateOnly)
            | (BaseType::String, PropertyFormat::TimeOnly)
            | (BaseType::String, PropertyFormat::TimeSpan) => true,
            _ => false,
        }
    }
}
