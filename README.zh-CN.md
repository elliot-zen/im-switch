# im-switch

[English](README.md) | 简体中文

一个轻量的 macOS 命令行工具，在启动终端应用前切换到英文输入源。

## 安装

计划支持 Homebrew 安装，Formula 发布后会补充具体安装命令。

<details>
<summary>从源码构建</summary>

本地构建需要 Rust 1.88+ 和 Xcode Command Line Tools。在项目目录执行，并确保 `~/.local/bin` 已加入 `PATH`。

```sh
cargo build --release --locked
mkdir -p "$HOME/.local/bin"
install -m 755 target/release/im-switch "$HOME/.local/bin/im-switch"
```

</details>

## 用法

先在「系统设置 → 键盘」中启用 ABC 或 U.S. 输入源，然后运行：

```sh
im-switch --english  # 切换到 ABC 或 U.S.
im-switch --status   # 输出当前输入源 ID
im-switch --help
```

`--english` 优先选择 ABC，其次 U.S.，确认切换成功后才返回成功。重复调用保持英文，无需辅助功能权限。

不带参数时仅显示帮助，不改变输入源。

## Herdr：lazygit 和 yazi

在 `~/.config/herdr/config.toml` 中添加以下浮窗快捷键。已有相同快捷键时，修改原配置即可。确保 Herdr 的 `PATH` 能找到这三个命令。

```toml
[[keys.command]]
key = "prefix+t"
type = "popup"
command = "im-switch --english && exec lazygit"
description = "Open Lazygit in English"
width = "90%"
height = "90%"

[[keys.command]]
key = "prefix+y"
type = "popup"
command = "im-switch --english && exec yazi"
description = "Open Yazi in English"
width = "90%"
height = "90%"
```

检查并重载配置：

```sh
herdr config check
herdr server reload-config
```

`&&` 确保先切换成功再启动应用。关闭浮窗后仍保留英文输入源。

## 说明

- 切换的是系统输入源，不控制鼠须管等输入法内部的中英文模式（如 `ascii_mode`）。
- 项目代码禁用 `unsafe`，通过 `cidre` 的安全接口调用系统 API；此限制不约束依赖库内部实现。

## 贡献

分支协作和发布流程见 [CONTRIBUTING.md](CONTRIBUTING.md)。项目采用 [MIT 许可证](LICENSE)。
