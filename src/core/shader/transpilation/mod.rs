use std::sync::LazyLock;

use anyhow::Result;
use spirv_cross2::{Module, targets::Glsl};

use crate::{bindings::frontend::gl, traits::shaderc::ToShaderType, utils::sync::UnsafeSendSync};

pub mod preprocess;

pub static COMPILER: LazyLock<shaderc::Compiler> = LazyLock::new(
    || shaderc::Compiler::new().unwrap()
);

static GLSL2SPV_OPTIONS: LazyLock<UnsafeSendSync<shaderc::CompileOptions>> = LazyLock::new(|| {
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.set_generate_debug_info();
    options.set_source_language(shaderc::SourceLanguage::GLSL);
    options.set_target_env(shaderc::TargetEnv::OpenGL, shaderc::EnvVersion::OpenGL4_5 as u32);
    
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);

    options.set_auto_map_locations(true);
    options.set_auto_bind_uniforms(true);

    UnsafeSendSync(options)
});

static SPV2ESSL_OPTIONS: LazyLock<spirv_cross2::compile::glsl::CompilerOptions> = LazyLock::new(|| {
    let mut options = spirv_cross2::compile::glsl::CompilerOptions::default();

    options.version = spirv_cross2::compile::glsl::GlslVersion::Glsl320Es;
    options.vulkan_semantics = false;
    options.enable_420pack_extension = false;
    options.force_flattened_io_blocks = true;
    options.common.enable_storage_image_qualifier_deduction = false;

    options
});

pub fn transpile(
    source: &str,
    shader_type: gl::types::GLenum,
    transpile_state: &mut super::TranspileState
) -> Result<String> {   
    // 1. glsl -> spv
    let spv = COMPILER.compile_into_spirv(
        source, 
        shader_type.to_shader_type()?,
        "shader", 
        "main", 
        Some(&*GLSL2SPV_OPTIONS)
    )?;

    // 2. spv -> essl
    let module = Module::from_words(spv.as_binary());    
    let mut compiler = spirv_cross2::Compiler::<Glsl>::new(module)?;

    preprocess::process_spv_bytecode(&mut compiler, shader_type, transpile_state)?;

    let artifact = compiler.compile(&*SPV2ESSL_OPTIONS)?;
    Ok(artifact.to_string())
}

