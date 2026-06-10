use gl_generator::{Api, Profile};

use crate::bindings::{BindingsBuilder, GeneratorType};

mod bindings;

fn main() {
    BindingsBuilder::default()
        .with_api(Api::Gles2, (3, 2), Profile::Core, GeneratorType::Struct)
        .with_api(Api::Egl, (1, 5), Profile::Core, GeneratorType::Struct)
        .write_to_separate_files("backend_{}.rs");


    BindingsBuilder::default()
        .with_api(Api::Gl, (3, 2), Profile::Compatibility, GeneratorType::Global)
        .with_api(Api::Glx, (1, 4), Profile::Core, GeneratorType::Global)
        .write_to_single_file("frontend_bindings.rs");


    println!("cargo:rerun-if-changed=build.rs");
}

