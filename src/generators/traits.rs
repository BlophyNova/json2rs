use crate::infer::types::TypeInferenceContext;
use crate::config::GeneratorConfig;
use anyhow::Result;

/// 代码生成器接口
pub trait CodeGenerator {
    /// 生成代码
    fn generate(
        &self,
        context: &TypeInferenceContext,
        root_name: &str,
        config: &GeneratorConfig,
    ) -> Result<String>;
}
