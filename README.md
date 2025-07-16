# BugShot - Bug 视频录制与操作日志工具

BugShot 是一个专为开发者和测试人员设计的桌面工具，支持录制屏幕视频、自动记录操作日志，并在录制过程中通过全局代理抓包网络请求，方便问题复现与分析。

## 主要功能

1. **视频录制**：一键录制屏幕操作，生成高质量视频，便于 bug 复现和沟通。
2. **操作日志记录**：自动捕捉并转换用户在视频中的操作（如鼠标点击、键盘输入等）为结构化文本日志，便于后续分析和追踪。
3. **全局代理抓包**：录制过程中自动开启全局代理，抓取所有网络请求，支持自定义过滤规则，精准定位问题请求。

## 安装方法

1. **克隆项目**

```bash
git clone <你的仓库地址>
cd bugshot
```

2. **安装依赖**

```bash
pnpm install
# 或者使用 npm install
```

3. **启动开发环境**

```bash
pnpm tauri dev
```

## 使用说明

1. 启动应用后，点击“开始录制”按钮即可录制屏幕。
2. 录制过程中，所有操作会被自动记录为文本日志。
3. 可在设置中配置全局代理的过滤规则，抓取所需的网络请求。
4. 录制结束后，视频和操作日志会自动保存，可用于 bug 复现和问题分析。

## 配置说明

- **代理过滤规则**：
  - 可在应用设置中自定义需要抓取或排除的请求类型、域名、路径等。
  - 支持正则表达式和多条件组合。
- **日志导出**：
  - 支持导出为文本文件，便于与团队成员共享。
- **视频格式**：
  - 默认导出为 mp4 格式。

## 推荐开发环境

- [VS Code](https://code.visualstudio.com/) + [Tauri 插件](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 贡献与反馈

欢迎提交 issue 或 PR 参与项目改进。如有建议或问题，请在 GitHub 上反馈。
