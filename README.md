# json2rs

This program is released under license GPLv3.

以json文件生成Rust结构体（实际不仅仅支持Rust）。
Generate Rust structs from a JSON file (actually supports more than just Rust).

## 安装

```bash
cargo install json2rs
```

## 用法:

```shell
json2rs <INPUT> [-c <rust|kotlin|jdk17|python|jsoncpp>] [-o <OUTPUT FILE>]
```

查看`json2rs -h`了解更多。

使用jsoncpp选项时请包含[这个](https://github.com/jamctl/jamctl/blob/master/jamd/ext/jsonSerialization.h)头文件。

## Installation

```bash
cargo install json2rs
```

## Usage:

```shell
json2rs <INPUT> [-c <rust|kotlin|jdk17|python|jsoncpp>] [-o <OUTPUT FILE>]
```

Run `json2rs -h` for more information.

When using jsoncpp options, please
include [this](https://github.com/jamctl/jamctl/blob/master/jamd/ext/jsonSerialization.h) header file.