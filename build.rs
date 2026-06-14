use gl_generator::{Api, Profile};

use crate::bindings::{BindingsBuilder, GeneratorType};

mod bindings;

fn main() {
    println!("cargo:rustc-link-lib=static=c++_static");
    println!("cargo:rustc-link-lib=static=c++abi");

    BindingsBuilder::default()
        .with_api(Api::Gles2, (3, 2), Profile::Core, GeneratorType::Struct)
        .with_api(Api::Egl, (1, 5), Profile::Core, GeneratorType::Struct)
        .with_api(Api::Gl, (4, 5), Profile::Compatibility, GeneratorType::Struct) 
        .write_to_separate_files("backend_{}.rs");

    println!("cargo:rerun-if-changed=build.rs");
}

