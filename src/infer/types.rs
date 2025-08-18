use std::collections::BTreeMap;
use serde_json::Value;
use crate::infer::structs::StructDef;

/// 推断的字段类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InferredType {
    Bool,
    I64,
    F64,
    String,
    Null,
    Array(Box<InferredType>),
    Object(String),
}

impl InferredType {
    /// 获取基础类型名称
    pub fn base_type_name(&self) -> &'static str {
        match self {
            InferredType::Bool => "bool",
            InferredType::I64 => "i64",
            InferredType::F64 => "f64",
            InferredType::String => "string",
            InferredType::Null => "null",
            InferredType::Array(_) => "array",
            InferredType::Object(_) => "object",
        }
    }

    /// 检查是否可为空
    pub fn is_nullable(&self) -> bool {
        matches!(self, InferredType::Null)
    }
}

/// 类型推断上下文
pub struct TypeInferenceContext {
    pub struct_defs: BTreeMap<String, StructDef>,
    pub nullable_fields: bool,
}

impl TypeInferenceContext {
    pub fn new(nullable_fields: bool) -> Self {
        Self {
            struct_defs: BTreeMap::new(),
            nullable_fields,
        }
    }

    /// 推断值的类型
    pub fn infer_type(
        &mut self,
        value: &Value,
        parent_name: &str,
        field_name: &str,
    ) -> InferredType {
        match value {
            Value::Null => InferredType::Null,
            Value::Bool(_) => InferredType::Bool,
            Value::Number(n) => {
                if n.is_i64() {
                    InferredType::I64
                } else {
                    InferredType::F64
                }
            }
            Value::String(_) => InferredType::String,
            Value::Array(arr) => {
                if arr.is_empty() {
                    InferredType::Array(Box::new(InferredType::Null))
                } else {
                    let element_type = self.infer_type(&arr[0], parent_name, field_name);
                    InferredType::Array(Box::new(element_type))
                }
            }
            Value::Object(obj) => {
                let child_name = format!("{}{}", parent_name, sanitize_struct_name(field_name));
                self.process_object(obj, &child_name);
                InferredType::Object(child_name)
            }
        }
    }

    /// 处理JSON对象
    pub fn process_object(&mut self, obj: &serde_json::Map<String, Value>, struct_name: &str) {
        if self.struct_defs.contains_key(struct_name) {
            return;
        }

        let mut struct_def = StructDef::default();

        for (key, value) in obj {
            let field_name = sanitize_field_name(key);
            let field_type = self.infer_type(value, struct_name, key);
            struct_def.fields.insert(field_name, field_type);
        }

        self.struct_defs.insert(struct_name.to_string(), struct_def);
    }
}

/// 清理结构体名称（驼峰命名）
pub fn sanitize_struct_name(name: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        } else {
            capitalize_next = true;
        }
    }

    // 确保名称是有效的标识符
    if result.is_empty() || result.chars().next().unwrap().is_ascii_digit() {
        result = format!("_{}", result);
    }

    result
}

/// 清理字段名称（蛇形命名）
pub fn sanitize_field_name(name: &str) -> String {
    let mut result = String::new();
    let mut last_was_upper = false;

    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_alphanumeric() {
            if c.is_ascii_uppercase() {
                if i > 0 && !last_was_upper {
                    result.push('_');
                }
                result.push(c.to_ascii_lowercase());
                last_was_upper = true;
            } else {
                result.push(c);
                last_was_upper = false;
            }
        } else {
            result.push('_');
            last_was_upper = false;
        }
    }

    // 处理关键字冲突
    if is_keyword(&result) {
        result.push('_');
    }

    result
}

/// 检查是否为关键字
fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else"
        | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop"
        | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static"
        | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while"
    )
}