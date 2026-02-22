use super::traits::CodeGenerator;
use crate::infer::types::{TypeInferenceContext, InferredType};
use crate::config::GeneratorConfig;
use anyhow::Result;

pub struct ConfigBasedGenerator;

impl CodeGenerator for ConfigBasedGenerator {
    fn generate(
        &self,
        context: &TypeInferenceContext,
        _root_name: &str,
        config: &GeneratorConfig,
    ) -> Result<String> {
        let mut output = String::new();

        // 添加文件头
        if let Some(header) = &config.file_header {
            output.push_str(header);
        }

        // 生成所有结构体
        for (name, def) in &context.struct_defs {
            output.push_str(&render_struct(name, def, config));
        }

        // 添加文件尾
        if let Some(footer) = &config.file_footer {
            output.push_str(footer);
        }

        Ok(output)
    }
}

/// 渲染结构体
fn render_struct(name: &str, def: &crate::infer::structs::StructDef, config: &GeneratorConfig) -> String {
    let indent = config.indent.as_deref().unwrap_or("");
    let field_sep = config.field_separator.as_deref().unwrap_or("\n");

    let mut parts = Vec::new();

    // 添加结构体前部
    if let Some(before_struct) = &config.before_struct {
        parts.push(before_struct.to_string());
    }

    // 添加结构体名称前部
    if let Some(before_name) = &config.before_struct_name {
        parts.push(before_name.to_string());
    }

    // 添加结构体名称
    parts.push(name.to_string());

    // 添加结构体名称后部
    if let Some(after_name) = &config.after_struct_name {
        parts.push(after_name.to_string());
    }

    // 添加字段
    let mut fields = Vec::new();
    for (field_name, field_type) in &def.fields {
        let field_type_str = map_field_type(field_type, config);
        fields.push(config.render_field(field_name, &field_type_str, indent));
    }
    parts.push(fields.join(field_sep));

    // 添加结构体后部
    if let Some(after_struct) = &config.after_struct {
        parts.push(after_struct.to_string());
    }

    parts.join("")
}

/// 映射字段类型
fn map_field_type(field_type: &InferredType, config: &GeneratorConfig) -> String {
    match field_type {
        InferredType::Array(inner) => {
            let inner_type = map_field_type(inner, config);
            config.map_type("array", false).replace("$T", &inner_type)
        }
        InferredType::Object(name) => {
            config.map_type("object", false).replace("$T", name)
        }
        _ => {
            let base_type = field_type.base_type_name();
            config.map_type(base_type, field_type.is_nullable())
        }
    }
}
