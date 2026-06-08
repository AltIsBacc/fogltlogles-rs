use std::{env, fs::File, io::Write, path::{Path, PathBuf}};
use gl_generator::{Api, Fallbacks, GlobalGenerator, StructGenerator, StaticGenerator, Profile, Registry};

/// Defines the target code output architecture for gl_generator
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GeneratorType {
    Global,
    Struct,
    Static,
}

/// Holds configuration parameters for a specific OpenGL API profile
pub struct ApiConfig {
    pub api: Api,
    pub version: (u8, u8),
    pub profile: Profile,
    pub generator: GeneratorType,
}

pub struct BindingsBuilder {
    out_dir: PathBuf,
    apis: Vec<ApiConfig>,
}

impl Default for BindingsBuilder {
    fn default() -> Self {
        BindingsBuilder::new(
            env::var("OUT_DIR")
                .map(PathBuf::from)
                .expect("No OUT_DIR set!")
        )
    }
}

impl BindingsBuilder {
    pub fn new<P: AsRef<Path>>(out_dir: P) -> Self {
        Self {
            out_dir: out_dir.as_ref().to_path_buf(),
            apis: Vec::new(),
        }
    }

    /// Appends an API with a configurable generator type
    pub fn with_api(
        mut self, 
        api: Api, 
        version: (u8, u8), 
        profile: Profile, 
        generator: GeneratorType
    ) -> Self {
        self.apis.push(ApiConfig {
            api,
            version,
            profile,
            generator,
        });
        self
    }

    pub fn write_to_single_file(self, filename: &str) {
        let mut file = File::create(self.out_dir.join(filename))
            .expect("Failed to create output file!");

        for config in self.apis {
            writeln!(file, "pub mod {} {{", Self::api_mod_name(config.api)).unwrap();
            writeln!(file, "    #![allow(unsafe_op_in_unsafe_fn)]").unwrap();
            Self::write_api_bindings(&config, &mut file);
            writeln!(file, "}}").unwrap();
        }
    }

    pub fn write_to_separate_files(self, filename_pattern: &str) {
        for config in self.apis {
            let filename = filename_pattern.replace("{}", Self::api_mod_name(config.api));
            let mut file = File::create(self.out_dir.join(&filename))
                .expect("Failed to create output file!");

            Self::write_api_bindings(&config, &mut file);
        }
    }

    fn write_api_bindings(config: &ApiConfig, file: &mut File) {
        let registry = Registry::new(config.api, config.version, config.profile, Fallbacks::All, []);

        match config.generator {
            GeneratorType::Global => registry.write_bindings(GlobalGenerator, file),
            GeneratorType::Struct => registry.write_bindings(StructGenerator, file),
            GeneratorType::Static => registry.write_bindings(StaticGenerator, file),
        }
        .expect("Failed to generate bindings!");
    }

    fn api_mod_name(api: Api) -> &'static str {
        match api {
            Api::Gl => "gl",
            Api::Gles1 => "gles1",
            Api::Gles2 => "gles2",
            Api::Glsc2 => "glsc2",
            Api::GlCore => "glcore",
            Api::Glx => "glx",
            Api::Wgl => "wgl",
            Api::Egl => "egl",
        }
    }
}

