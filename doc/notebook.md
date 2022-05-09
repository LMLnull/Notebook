



### Cargo

#### Why Cargo

Cargo 是一个可以让 Rust 项目声明其各种依赖并保证可一直重复构建的工具。

为了达到这个目标，Cargo 做了四件事：

- 引入两个包含项目各种信息的元数据文件。
- 获取并构建项目依赖。
- 调用 `rustc` 或其他构建工具并使用正确的构建参数来构建项目。
- 引入简便性以便在 Rust 项目上工作更加容易。

#### Cargo新建项目

```shell
// 新建项目，会自动生成相应目录和git文件   vcs：version control system
cargo new project_name [--lib/--bin/--vcs none]     
// 编译整个项目
cargo build
// 编译 + 运行
cargo run
// 打开优化开关
cargo build --release
```

Cargo.toml被称为manifest，其包括了 Cargo 编译项目的所有元数据。

#### Cargo添加依赖并使用

[crate](https://crates.io/)是 Rust 社区的包注册中心，其可用于发现和下载包的网址。`cargo` 将它用作发现所需包的默认配置。

添加两个依赖包 time和regex如下：

```
[dependencies]
time = "0.1.12"
regex = "0.1.41"
```

在cargo build的时候，会拉取新的依赖以及依赖包的依赖，编译然后更`Cargo.lock`，`Cargo.lock` 包含所有所依赖的修订版的确切信息。使用依赖的包：

```rust
extern crate regex;
use regex::Regex;
fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
```

#### Cargo管理的项目布局

```
.
├── Cargo.lock
├── Cargo.toml
├── benches
│   └── large-input.rs
├── examples
│   └── simple.rs
├── src
│   ├── bin
│   │   └── another_executable.rs
│   ├── lib.rs
│   └── main.rs
└── tests
    └── some-integration-tests.rs
```

- `Cargo.toml` 和 `Cargo.lock` 存储在项目根目录下（*包目录*）。
- 源代码位于 `src` 目录。
- 默认库文件为 `src/lib.rs` 。
- 默认可执行文件为 `src/main.rs` 。
- 其他可执行文件可以放于 `src/bin/*.rs` 。
- 集成测试位于 `tests` 目录（单元测试放在每个文件中。）
- 示例文件位于 `examples` 目录。
- Benchmark 位于 `benches` 目录。

### 问题

#### 1. 安装rustup提示权限问题：could not write rcfile file

即使使用sudo也无权限，报错如下：

```
error: could not amend shell profile: '/Users/limingliang/.profile': could not write rcfile file: '/Users/limingliang/.profile': Permission denied (os error 13)
```

stackoverflow上有[解决方案](https://stackoverflow.com/questions/45899815/could-not-write-to-bash-profile-when-installing-rust-on-macos-sierra)

Give a try using this not using sudo:

```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

If that works then probably you could try:

```
curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path 
```

If the command with the *--no-modify-path* option works, you'll have to manually update **.bash_profile** to include it in your path:

```
source ~/.cargo/env
```

