use super::{
    property::{Property, PropertyFormat, PropertyOptions},
    GeneratorPartial,
};
use crate::google::protobuf::{compiler::CodeGeneratorRequest, field_descriptor_proto, DescriptorProto, EnumDescriptorProto, FieldDescriptorProto};
use convert_case::{Case, Casing};
use std::{borrow::Borrow, collections::HashSet, hash::Hash, ops::Deref, rc::Rc};

pub struct TypeMap {
    records: HashSet<TypeRecordRef>,
}

impl TypeMap {
    pub fn new(request: &CodeGeneratorRequest) -> Self {
        let mut this = TypeMap { records: HashSet::new() };
        this.insert(field_descriptor_proto::Type::Bool.as_str_name(), None, None, BaseType::Bool, false);
        this.insert(field_descriptor_proto::Type::Int32.as_str_name(), None, None, BaseType::Int, false);
        this.insert(field_descriptor_proto::Type::Int64.as_str_name(), None, None, BaseType::Long, false);
        this.insert(field_descriptor_proto::Type::Uint32.as_str_name(), None, None, BaseType::Uint, false);
        this.insert(field_descriptor_proto::Type::Uint64.as_str_name(), None, None, BaseType::ULong, false);
        this.insert(field_descriptor_proto::Type::Sint32.as_str_name(), None, None, BaseType::Int, false);
        this.insert(field_descriptor_proto::Type::Sint64.as_str_name(), None, None, BaseType::Long, false);
        this.insert(field_descriptor_proto::Type::Fixed32.as_str_name(), None, None, BaseType::Uint, false);
        this.insert(field_descriptor_proto::Type::Fixed64.as_str_name(), None, None, BaseType::ULong, false);
        this.insert(field_descriptor_proto::Type::Sfixed32.as_str_name(), None, None, BaseType::Int, false);
        this.insert(field_descriptor_proto::Type::Sfixed64.as_str_name(), None, None, BaseType::Long, false);
        this.insert(field_descriptor_proto::Type::Float.as_str_name(), None, None, BaseType::Float, false);
        this.insert(field_descriptor_proto::Type::Double.as_str_name(), None, None, BaseType::Double, false);
        this.insert(field_descriptor_proto::Type::Bytes.as_str_name(), None, None, BaseType::Bytes, false);
        this.insert(field_descriptor_proto::Type::String.as_str_name(), None, None, BaseType::String, false);
        this.insert(".google.protobuf.BoolValue", None, None, BaseType::Bool, true);
        this.insert(".google.protobuf.Int32Value", None, None, BaseType::Int, true);
        this.insert(".google.protobuf.Int64Value", None, None, BaseType::Long, true);
        this.insert(".google.protobuf.UInt32Value", None, None, BaseType::Uint, true);
        this.insert(".google.protobuf.UInt64Value", None, None, BaseType::ULong, true);
        this.insert(".google.protobuf.FloatValue", None, None, BaseType::Float, true);
        this.insert(".google.protobuf.DoubleValue", None, None, BaseType::Double, true);
        this.insert(".google.protobuf.BytesValue", None, None, BaseType::Bytes, true);
        this.insert(".google.protobuf.StringValue", None, None, BaseType::String, true);

        for file in &request.proto_file {
            let package = file.package();
            let namespace = GeneratorPartial::namespace(file);
            for enum_type in &file.enum_type {
                this.parse_enum(package, Some(&namespace), None, enum_type);
            }
            for message_type in &file.message_type {
                this.parse_message(package, Some(&namespace), None, message_type);
            }
        }

        for file in &request.proto_file {
            let package = file.package();
            let namespace = GeneratorPartial::namespace(file);
            for message_type in &file.message_type {
                this.parse_map(package, Some(&namespace), None, message_type);
            }
        }

        this
    }

    pub fn proto_name(field: &FieldDescriptorProto) -> &str {
        match field.r#type() {
            field_descriptor_proto::Type::Enum | field_descriptor_proto::Type::Message => field.type_name(),
            _ => field.r#type().as_str_name(),
        }
    }

    pub fn insert(&mut self, proto_name: &str, namespace: Option<&str>, parent_name: Option<&str>, base_type: BaseType, nullable: bool) {
        if !self.records.contains(proto_name) {
            self.records.insert(TypeRecordRef::new(proto_name, namespace, parent_name, base_type, nullable));
        }
    }

    pub fn get(&self, proto_name: &str) -> TypeRecordRef {
        self.records.get(proto_name).and_then(|r| Some(r.clone())).expect(&format!("unknown type: {}", proto_name))
    }

    fn parse_enum(&mut self, package: &str, namespace: Option<&str>, parent_name: Option<&str>, enum_type: &EnumDescriptorProto) {
        let proto_name = format!(".{}.{}", package, enum_type.name());
        let enum_name = enum_type.name().to_case(Case::Pascal);
        self.insert(&proto_name, namespace, parent_name, BaseType::Enum(enum_name), false);
    }

    fn parse_message(&mut self, package: &str, namespace: Option<&str>, parent_name: Option<&str>, message_type: &DescriptorProto) {
        let proto_name = format!(".{}.{}", package, message_type.name());
        let class_name = message_type.name().to_case(Case::Pascal);
        let map_entry = message_type.options.as_ref().is_some_and(|o| o.map_entry());
        if !map_entry {
            self.insert(&proto_name, namespace, parent_name, BaseType::Message(class_name.to_string()), true);
        }
        let package = format!("{}.{}", package, message_type.name());
        let parent_name = parent_name.and_then(|p| Some(format!("{}.{}", p, class_name))).unwrap_or(class_name);
        for nested_type in &message_type.enum_type {
            self.parse_enum(&package, namespace, Some(&parent_name), nested_type);
        }
        for nested_type in &message_type.nested_type {
            self.parse_message(&package, namespace, Some(&parent_name), nested_type);
        }
    }

    fn parse_map(&mut self, package: &str, namespace: Option<&str>, parent_name: Option<&str>, message_type: &DescriptorProto) {
        let proto_name = format!(".{}.{}", package, message_type.name());
        let class_name = message_type.name().to_case(Case::Pascal);
        let map_entry = message_type.options.as_ref().is_some_and(|o| o.map_entry());
        if map_entry {
            let key_type_record = message_type
                .field
                .get(0)
                .and_then(|f| Some(self.get(Self::proto_name(f))))
                .expect(&format!("unknown key field: {}", message_type.name()));
            let value_type_record = message_type
                .field
                .get(1)
                .and_then(|f| Some(self.get(Self::proto_name(f))))
                .expect(&format!("unknown value field: {}", message_type.name()));
            self.insert(&proto_name, namespace, parent_name, BaseType::Map(key_type_record, value_type_record), false);
        }
        let package = format!("{}.{}", package, message_type.name());
        let parent_name = parent_name.and_then(|p| Some(format!("{}.{}", p, class_name))).unwrap_or(class_name);
        for nested_type in &message_type.nested_type {
            self.parse_map(&package, namespace, Some(&parent_name), nested_type);
        }
    }

    pub fn property(&self, field: &FieldDescriptorProto) -> Property {
        let type_record = self.get(Self::proto_name(field));
        let repeated = field.label() == field_descriptor_proto::Label::Repeated && !matches!(type_record.base_type, BaseType::Map(..));
        let name = field.name().to_case(Case::Pascal);
        let mut options = PropertyOptions::default();
        if let Some(o) = &field.options {
            options.required = o.required();
            options.eq = o.eq;
            options.ne = o.ne;
            options.lt = o.lt;
            options.lte = o.lte;
            options.gt = o.gt;
            options.gte = o.gte;
            options.len = o.len;
            options.min_len = o.min_len;
            options.max_len = o.max_len;
            options.count = o.count;
            options.min_count = o.min_count;
            options.max_count = o.max_count;
            options.key_format = PropertyFormat::from_proto(&o.key_format());
            options.format = PropertyFormat::from_proto(&o.format());
        };
        Property::new(type_record, repeated, &name, &options)
    }
}

pub struct TypeRecord {
    pub proto_name: String,
    pub namespace: Option<String>,
    pub parent_name: Option<String>,
    pub base_type: BaseType,
    pub nullable: bool,
}

pub struct TypeRecordRef {
    inner: Rc<TypeRecord>,
}

impl TypeRecordRef {
    pub fn new(proto_name: &str, namespace: Option<&str>, parent_name: Option<&str>, base_type: BaseType, nullable: bool) -> Self {
        TypeRecordRef {
            inner: Rc::new(TypeRecord {
                proto_name: proto_name.to_string(),
                namespace: namespace.map(|s| s.to_string()),
                parent_name: parent_name.map(|s| s.to_string()),
                base_type,
                nullable,
            }),
        }
    }
}

impl Deref for TypeRecordRef {
    type Target = TypeRecord;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for TypeRecordRef {
    fn clone(&self) -> Self {
        TypeRecordRef { inner: Rc::clone(&self.inner) }
    }
}

impl Eq for TypeRecordRef {}

impl PartialEq for TypeRecordRef {
    fn eq(&self, other: &Self) -> bool {
        self.proto_name == other.proto_name
    }
}

impl Hash for TypeRecordRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.proto_name.hash(state);
    }
}

impl Borrow<str> for TypeRecordRef {
    fn borrow(&self) -> &str {
        &self.proto_name
    }
}

#[derive(Clone)]
pub enum BaseType {
    Bool,
    Int,
    Long,
    Uint,
    ULong,
    Float,
    Double,
    Bytes,
    String,
    Enum(String),
    Message(String),
    Map(TypeRecordRef, TypeRecordRef),
}
