pub use crate::generators::config_based::ConfigBasedGenerator;
pub use crate::generators::traits::CodeGenerator;

pub mod config_based;
pub mod traits;

pub fn get_generator() -> Box<dyn CodeGenerator> {
    Box::new(ConfigBasedGenerator)
}
