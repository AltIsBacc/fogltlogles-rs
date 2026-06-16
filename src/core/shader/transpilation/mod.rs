use std::sync::LazyLock;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use spirv_cross2::{Module, targets::Glsl};

use crate::{bindings::frontend::gl, traits::shaderc::ToShaderType, utils::sync::UnsafeSendSync};

pub mod preprocess;

type GxHashMap<K, V> = gxhash::HashMap<K, V>;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct TranspileContext {
    pub bindings: GxHashMap<String, gl::types::GLuint>,

    #[serde(skip, default)]
    pub next_binding: u32,
}

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

pub fn glsl_to_spv(
    source: &str,
    shader_type: gl::types::GLenum,
) -> Result<Vec<u8>> {
    let artifact = COMPILER.compile_into_spirv(
        source,
        shader_type.to_shader_type()?,
        "shader",
        "main",
        Some(&*GLSL2SPV_OPTIONS),
    )?;
    Ok(artifact.as_binary_u8().to_vec())
}

pub fn spv_to_essl(
    spv: &[u8],
    shader_type: gl::types::GLenum,
    transpile_ctx: &mut TranspileContext,
) -> Result<String> {
    let module = Module::from_words(bytemuck::cast_slice(spv));
    let mut compiler = spirv_cross2::Compiler::<Glsl>::new(module)?;

    preprocess::process_spv_bytecode(&mut compiler, shader_type, transpile_ctx)?;

    let artifact = compiler.compile(&*SPV2ESSL_OPTIONS)?;
    Ok(artifact.to_string())
}

pub fn transpile(source: &str, shader_type: gl::types::GLenum, ctx: &mut TranspileContext) -> Result<String> {
    spv_to_essl(&glsl_to_spv(source, shader_type)?, shader_type, ctx)
}

