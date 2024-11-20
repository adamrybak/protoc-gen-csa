use crate::google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
pub use generator_common::*;
pub use generator_model::*;
pub use generator_partial::*;
pub use property::*;
pub use type_map::*;

mod generator_common;
mod generator_model;
mod generator_partial;
mod property;
mod type_map;

pub struct Generator<'a> {
    request: &'a CodeGeneratorRequest,
    response: &'a mut CodeGeneratorResponse,
    type_map: &'a TypeMap,
}

impl<'a> Generator<'a> {
    pub fn generate(request: &CodeGeneratorRequest, response: &mut CodeGeneratorResponse, type_map: &TypeMap) {
        let mut this = Generator { request, response, type_map };

        GeneratorCommon::generate(&mut this.response);

        for file in &this.request.proto_file {
            if request.file_to_generate.contains(&file.name().to_string()) {
                GeneratorPartial::generate(&mut this.response, &this.type_map, file);
            }
        }
    }
}
