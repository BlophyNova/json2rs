use clap::Parser;
use std::path::PathBuf;

/// 从JSON文件生成结构体定义。
#[derive(Parser, Debug)]
#[command(version, about, long_about = "https://github.com/BlophyNova/json2rs")]
pub struct Cli {
    /// 输入的JSON文件路径
    #[arg(name = "input")]
    pub input: PathBuf,

    /// 输出的文件路径
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// 根结构体名称
    #[arg(short = 'r', long, default_value = "Root")]
    pub root_name: String,

    /// 是否使用可选字段
    #[arg(short = 'n', long, default_value_t = false)]
    pub nullable_fields: bool,

    /// 配置文件名称/路径
    #[arg(short = 'c', long, default_value = "rust")]
    pub config: Option<PathBuf>,
}
