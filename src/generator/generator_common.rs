use crate::google::protobuf::compiler::{code_generator_response, CodeGeneratorResponse};

pub struct GeneratorCommon {}

impl GeneratorCommon {
    pub fn generate(response: &mut CodeGeneratorResponse) {
        let file_name = Self::file_name().to_string();
        let content = Self::write_content().to_string();
        response.file.push(code_generator_response::File {
            name: Some(file_name),
            content: Some(content),
            insertion_point: None,
            generated_code_info: None,
        });
    }

    pub fn file_name() -> &'static str {
        "CsaCommon.cs"
    }

    fn write_content() -> &'static str {
        include_str!("assets/CsaCommon.cs")
    }
}
