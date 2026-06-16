use std::hash::Hasher;

use gxhash::GxHasher;

use crate::{core::{contexts::es::ESContext, shader::{Program, Shader}}, traits::gl::FromStageKey};

pub fn spv_key(
    shader: &Shader
) ->u64 {
    let mut hasher = GxHasher::default();

    hasher.write(&shader.type_.to_le_bytes());
    hasher.write(shader.source.as_bytes());

    hasher.finish()
}

pub fn program_key(
    ctx: &ESContext,
    program: &Program,
) -> u64 {
    let mut hasher = GxHasher::default();

    hasher.write(&ctx.version.count_bytes().to_le_bytes());
    hasher.write(ctx.version.to_bytes());
    hasher.write(&ctx.renderer.count_bytes().to_le_bytes());
    hasher.write(ctx.renderer.to_bytes());

    for (i, stage) in program.stages.iter().enumerate() {
        hasher.write(&i.from_stage_key().unwrap().to_le_bytes());
        for source in &stage.shaders {
            hasher.write(&source.to_ne_bytes());
        }
    }

    hasher.finish()
}

