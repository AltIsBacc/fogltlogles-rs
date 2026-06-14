use anyhow::Result;
use spirv_cross2::{Compiler, reflect::{DecorationValue, ResourceType, ShaderResources}, spirv::Decoration, targets::Glsl};

use crate::bindings::frontend::gl;
use crate::core::shader::TranspileState;

const ALL_DECORATIONS: &[Decoration] = &[
    Decoration::Location,
    Decoration::Binding,
    Decoration::DescriptorSet,
];

const NO_LOCATION: &[Decoration] = &[
    Decoration::Binding,
    Decoration::DescriptorSet,
];

fn unset_decorations(
    compiler: &mut Compiler<Glsl>,
    resource_type: ResourceType,
    resources: &ShaderResources,
    decorations: &[Decoration],
) -> Result<()> {
    for res in resources.resources_for_type(resource_type)? {
        for &decoration in decorations {
            compiler.set_decoration(res.id, decoration, DecorationValue::unset())?;
        }
    }
    Ok(())
}

fn assign_uniform_buffer_bindings(
    compiler: &mut Compiler<Glsl>,
    resources: &ShaderResources,
    state: &mut TranspileState,
) -> Result<()> {
    for res in resources.resources_for_type(ResourceType::UniformBuffer)? {
        let name = res.name.to_string();

        let binding = if let Some(&existing) = state.bindings.get(&name) {
            existing
        } else {
            let new_binding = state.next_binding;
            state.next_binding += 1;
            state.bindings.insert(name, new_binding);
            new_binding
        };

        compiler.set_decoration(res.id, Decoration::Binding, Some(DecorationValue::Literal(binding)))?;
    }
    Ok(())
}

pub fn process_spv_bytecode(
    compiler: &mut Compiler<Glsl>,
    shader_type: gl::types::GLenum,
    state: &mut TranspileState,
) -> Result<()> {
    if shader_type == gl::COMPUTE_SHADER {
        return Ok(());
    }

    let resources = compiler.shader_resources()?;

    unset_decorations(compiler, ResourceType::SampledImage, &resources, ALL_DECORATIONS)?;
    unset_decorations(compiler, ResourceType::SeparateImage, &resources, ALL_DECORATIONS)?;
    unset_decorations(compiler, ResourceType::SeparateSamplers, &resources, ALL_DECORATIONS)?;
    unset_decorations(compiler, ResourceType::GlPlainUniform, &resources, ALL_DECORATIONS)?;
    unset_decorations(compiler, ResourceType::StageInput, &resources, ALL_DECORATIONS)?;

    unset_decorations(compiler, ResourceType::UniformBuffer, &resources, NO_LOCATION)?;
    assign_uniform_buffer_bindings(compiler, &resources, state)?;

    let keep_location = shader_type == gl::FRAGMENT_SHADER
        && resources.resources_for_type(ResourceType::StageOutput)?.count() > 1;

    unset_decorations(
        compiler,
        ResourceType::StageOutput,
        &resources,
        if keep_location { NO_LOCATION } else { ALL_DECORATIONS },
    )?;

    Ok(())
}

