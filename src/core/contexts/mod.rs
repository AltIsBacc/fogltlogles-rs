pub mod management;
pub mod fogle;
pub mod es;
pub mod gl;

#[derive(Default)]
pub struct TranslationContext {
    pub gl: gl::GLContext,
    pub es: es::ESContext,
    pub fogle: fogle::FogleContext,
}

impl TranslationContext {
    pub fn new() -> Self {
        Self::default()
    }
}

