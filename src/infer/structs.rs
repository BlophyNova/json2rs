use std::collections::BTreeMap;
use super::types::InferredType;

/// 结构体定义
#[derive(Debug, Default)]
pub struct StructDef {
    pub fields: BTreeMap<String, InferredType>,
}