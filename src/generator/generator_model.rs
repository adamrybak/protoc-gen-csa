use super::TypeMap;
use crate::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};

pub struct GeneratorModel {}

impl GeneratorModel {
    pub fn generate(request: &CodeGeneratorRequest, type_map: &TypeMap, response: &mut CodeGeneratorResponse) {
        // todo
    }
}
