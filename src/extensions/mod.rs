
pub mod macros;

pub mod ext_blend_func_extended;
pub mod ext_multi_draw_arrays;

pub struct ExtensionEntry {
    pub register: fn(),
}

unsafe impl Sync for ExtensionEntry { }

#[linkme::distributed_slice]
pub static EXTENSION_REGISTRY: [ExtensionEntry];

