use generator::Generator;
use google::protobuf::compiler::CodeGeneratorRequest;
use prost::Message;
use std::io::{self, Read, Write};

pub mod generator;
pub mod utility;
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
    let response = Generator::generate(request);
    let mut buf = Vec::new();
    response.encode(&mut buf)?;
    io::stdout().write_all(&buf)?;
    Ok(())
}
