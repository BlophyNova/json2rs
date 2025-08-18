mod cli;
mod config;
mod generators;
mod infer;

use crate::infer::types::{TypeInferenceContext, sanitize_struct_name};
use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use config::GeneratorConfig;
use generators::{get_generator};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.input)
        .with_context(|| format!("无法打开文件: {}", args.input.display()))?;
    let reader = BufReader::new(file);
    let value: Value = serde_json::from_reader(reader)
        .with_context(|| format!("无效的JSON文件: {}", args.input.display()))?;

    let config = GeneratorConfig::load(args.config.as_deref())?;

    let mut context = TypeInferenceContext::new(args.nullable_fields);
    let root_name = sanitize_struct_name(&args.root_name);

    match &value {
        Value::Object(obj) => {
            context.process_object(obj, &root_name);
        }
        Value::Array(arr) if !arr.is_empty() => {
            if let Some(Value::Object(first_obj)) = arr.first() {
                context.process_object(first_obj, &root_name);
            }
        }
        _ => anyhow::bail!("JSON必须是对象或非空对象数组"),
    }

    let generator = get_generator();

    let code = generator
        .generate(&context, &root_name, &config)
        .context("生成代码失败")?;

    let output_path = match args.output {
        Some(path) => path,
        None => {
            let ext = config.file_extension();
            let mut path = args.input.clone();
            path.set_extension(ext);
            path
        }
    };

    let mut file = File::create(&output_path)
        .with_context(|| format!("无法创建文件: {}", output_path.display()))?;
    file.write_all(code.as_bytes())
        .with_context(|| format!("写入文件失败: {}", output_path.display()))?;

    println!("成功生成: {}", output_path.display());
    Ok(())
}
