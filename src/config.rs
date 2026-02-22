use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use std::path::Path;

lazy_static! {
    static ref BUILTIN_CONFIGS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("rust", include_str!("../configs/rust.toml"));
        m.insert("jsoncpp", include_str!("../configs/jsoncpp.toml"));
        m.insert("java", include_str!("../configs/jdk17.toml"));
        m.insert("python", include_str!("../configs/python.toml"));
        m.insert("kotlin", include_str!("../configs/kotlin.toml"));
        m
    };
}

/// 代码生成配置
#[derive(Debug, Deserialize, Default)]
pub struct GeneratorConfig {
    pub before_struct: Option<String>,
    pub after_struct: Option<String>,
    pub before_struct_name: Option<String>,
    pub after_struct_name: Option<String>,
    pub left_brace_replace_by: Option<String>,
    pub right_brace_replace_by: Option<String>,
    pub each_attr_format: Option<String>,
    pub number: Option<String>,
    pub string: Option<String>,
    pub boolean: Option<String>,
    pub array: Option<String>,
    pub object: Option<String>,
    pub null: Option<String>,
    pub optional: Option<String>,
    pub file_header: Option<String>,
    pub file_footer: Option<String>,
    pub indent: Option<String>,
    pub field_separator: Option<String>,
}

impl GeneratorConfig {
    /// 加载配置
    pub fn load(config: Option<&Path>) -> Result<Self> {
        if let Some(content) = BUILTIN_CONFIGS.get(config.unwrap().to_str().unwrap()) {
            return toml::from_str(content).context("解析内置配置失败");
        }

        let configs_dir_path = format!("configs/{}.toml", config.unwrap().to_str().unwrap());
        if Path::new(&configs_dir_path).exists() {
            let content = fs::read_to_string(&configs_dir_path)
                .with_context(|| format!("无法读取配置文件: {}", configs_dir_path))?;
            return toml::from_str(&content).context("解析配置文件失败");
        }

        if let Some(path) = config {
            let content = fs::read_to_string(path)
                .with_context(|| format!("无法读取配置文件: {}", path.display()))?;
            return toml::from_str(&content).context("解析配置文件失败");
        }

        anyhow::bail!("找不到语言配置: {}", config.unwrap().to_str().unwrap())
    }

    /// 应用替换规则
    fn apply_replacements(&self, s: &str) -> String {
        let mut result = s.to_string();

        if let Some(replacement) = &self.left_brace_replace_by {
            result = result.replace("{", replacement);
        }

        if let Some(replacement) = &self.right_brace_replace_by {
            result = result.replace("}", replacement);
        }

        result
    }

    /// 渲染字段
    pub fn render_field(&self, name: &str, field_type: &str, indent: &str) -> String {
        let mut result = if let Some(format) = &self.each_attr_format {
            format
                .replace("$NAME", name)
                .replace("$TYPE", field_type)
        } else {
            format!("{}{}: {}", indent, name, field_type)
        };

        result = self.apply_replacements(&result);
        result
    }

    /// 渲染类型
    pub fn map_type(&self, base_type: &str, is_nullable: bool) -> String {
        let type_str = match base_type {
            "bool" => self.boolean.as_deref().unwrap_or("bool"),
            "i64" | "f64" => self.number.as_deref().unwrap_or("number"),
            "string" => self.string.as_deref().unwrap_or("string"),
            "array" => self.array.as_deref().unwrap_or("array"),
            "object" => self.object.as_deref().unwrap_or("object"),
            "null" => self.null.as_deref().unwrap_or("null"),
            _ => base_type,
        };

        let result = if is_nullable {
            if let Some(opt_template) = &self.optional {
                opt_template.replace("$T", type_str)
            } else {
                format!("Option<{}>", type_str)
            }
        } else {
            type_str.to_string()
        };

        self.apply_replacements(&result)
    }
}
