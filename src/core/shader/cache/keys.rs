use std::hash::Hasher;

use gxhash::GxHasher;

use crate::{core::{ contexts::es::ESContext, shader::{Program, Shader, transpilation::TranspileContext}}, traits::gl::FromStageKey};

const HASH_SEED: i64 = 694206721;

pub fn spv_key(
    shader: &Shader
) ->u64 {
    let mut hasher = GxHasher::with_seed(HASH_SEED);

    hasher.write(&TranspileContext::VERSION.to_le_bytes());

    hasher.write(&shader.type_.to_le_bytes());
    hasher.write(shader.source.as_bytes());

    hasher.finish()
}

pub fn program_key(
    ctx: &ESContext,
    program: &Program,
) -> u64 {
    let mut hasher = GxHasher::with_seed(HASH_SEED);

    let version_bytes = ctx.version.as_bytes();
    hasher.write(&(version_bytes.len() as u64).to_le_bytes());
    hasher.write(version_bytes);
    
    let renderer_bytes = ctx.renderer.as_bytes(); 
    hasher.write(&(renderer_bytes.len() as u64).to_le_bytes());
    hasher.write(renderer_bytes);

    let shading_language_version_bytes = ctx.shading_language_version.as_bytes();
    hasher.write(&(shading_language_version_bytes.len() as u64).to_le_bytes());
    hasher.write(shading_language_version_bytes);

    for (i, stage) in program.stages.iter().enumerate() {
        hasher.write(&i.from_stage_key().unwrap().to_le_bytes());
        for source in &stage.shaders {
            hasher.write(&source.to_ne_bytes());
        }
    }

    hasher.finish()
}

