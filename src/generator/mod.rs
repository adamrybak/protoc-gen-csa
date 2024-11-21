use crate::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use generator_common::GeneratorCommon;
use generator_model::GeneratorModel;
pub use generator_partial::*;
pub use type_map::*;

mod generator_common;
mod generator_model;
mod generator_partial;
mod property;
mod type_map;

pub struct Generator {
    request: CodeGeneratorRequest,
    response: CodeGeneratorResponse,
    type_map: TypeMap,
}

impl Generator {
    pub fn generate(request: CodeGeneratorRequest) -> CodeGeneratorResponse {
        let response = CodeGeneratorResponse::default();
        let type_map = TypeMap::new(&request);
        let mut this = Generator { request, response, type_map };

        GeneratorCommon::generate(&mut this.response);

        for file in &this.request.proto_file {
            if this.request.file_to_generate.contains(&file.name().to_string()) {
                GeneratorPartial::generate(&mut this.response, &this.type_map, file);
                GeneratorModel::generate(&mut this.response, &this.type_map, file);
            }
        }

        this.response
    }
}
