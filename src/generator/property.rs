use super::{BaseType, TypeRecord};
use crate::google::protobuf::field_options;

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

pub struct Property<'a> {
    type_record: &'a TypeRecord,
    repeated: bool,
    name: String,
    options: PropertyOptions,
}

impl<'a> Property<'a> {
    pub fn new(type_record: &'a TypeRecord, repeated: bool, name: &str, options: &PropertyOptions) -> Self {
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

    pub fn type_name(&self) -> String {
        if let Some(parent_name) = &self.type_record.parent_name {
            format!("{}.{}", parent_name, self.nested_type_without_parent())
        } else {
            self.nested_type_without_parent()
        }
    }

    pub fn nested_type_without_parent(&self) -> String {
        let mut type_name = match &self.type_record.base_type {
            BaseType::Bool => "bool".to_string(),
            BaseType::Int => "int".to_string(),
            BaseType::Long => "long".to_string(),
            BaseType::Uint => "uint".to_string(),
            BaseType::ULong => "ulong".to_string(),
            BaseType::Float => "float".to_string(),
            BaseType::Double => "double".to_string(),
            BaseType::Bytes => "Google.Protobuf.ByteString".to_string(),
            BaseType::String => "string".to_string(),
            BaseType::Enum(x) => x.to_string(),
            BaseType::Message(x) => x.to_string(),
            BaseType::Map(k, v) => {
                let key_property = Property::new(k, false, "key", &self.options);
                let value_property = Property::new(v, false, "value", &self.options);
                format!("System.Collections.Generic.Dictionary<{}, {}>", key_property.type_name(), value_property.type_name())
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
        if let Some(namespace) = self.namespace().take_if(|x| *x != current_namespace) {
            format!("{}.{}", namespace, self.type_name())
        } else {
            self.type_name()
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
        if self.nullable() {
            None
        } else if self.repeated() || matches!(self.type_record.base_type, BaseType::Map(..)) {
            Some("[]")
        } else {
            None
        }
    }

    pub fn options(&self) -> &PropertyOptions {
        &self.options
    }
}
