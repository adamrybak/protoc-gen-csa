use super::TypeMap;
use crate::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};

pub struct GeneratorPartial {}

impl GeneratorPartial {
    pub fn generate(request: &CodeGeneratorRequest, type_map: &TypeMap, response: &mut CodeGeneratorResponse) {
        // todo
    }
}
