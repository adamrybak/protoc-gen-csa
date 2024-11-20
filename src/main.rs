use generator::{GeneratorModel, GeneratorPartial, TypeMap};
use google::protobuf::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use prost::Message;
use std::io::{self, Read, Write};

pub mod generator;
pub mod google {
    pub mod protobuf {
        pub mod compiler {
            include!(concat!(env!("OUT_DIR"), "/google.protobuf.compiler.rs"));
        }
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
    }
}

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let request = CodeGeneratorRequest::decode(buf.as_slice())?;
    let mut response = CodeGeneratorResponse::default();

    let type_map = TypeMap::new(&request);
    GeneratorPartial::generate(&request, &type_map, &mut response);
    GeneratorModel::generate(&request, &type_map, &mut response);

    let mut buf = Vec::new();
    response.encode(&mut buf)?;
    io::stdout().write_all(&buf)?;
    Ok(())
}
