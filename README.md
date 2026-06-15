# MyToDo

一款将待办清单像壁纸一样"贴"在桌面上的 Windows 桌面应用。简约、透明、有质感。

- **主界面**：事项的增删改查、过滤、搜索、软件设置、关于。可隐藏到系统托盘。
- **桌面悬浮层**：半透明/全透明窗口，按时间顺序展示未完成事项；锁定态鼠标穿透融入壁纸，解锁态可拖拽缩放。

技术栈：**Tauri 2 + Vue 3 + TypeScript**。前端 WebView2（Win11 预装），后端 Rust 调用 Win32 实现桌面层。详见 [设计规范.md](设计规范.md) 与 [需求文档.md](需求文档.md)。

---

## 目录结构

```
my_todo/
├─ index.html / desktop.html     # 主界面、桌面层两个入口
├─ src/                          # 前端 (Vue3 + TS)
│  ├─ api.ts                     # 桥接层（Tauri 命令 / 浏览器 mock）
│  ├─ store.ts  types.ts  utils.ts
│  ├─ App.vue  main.ts  desktop.ts
│  └─ components/                # 各界面组件 + 桌面层
├─ src-tauri/                    # 后端 (Rust)
│  ├─ src/
│  │  ├─ lib.rs                  # 应用入口、单实例、窗口事件、置底守护
│  │  ├─ commands.rs             # Tauri 命令 + 桌面层控制
│  │  ├─ storage.rs              # 数据模型 + JSON 持久化（容错）
│  │  ├─ tray.rs                 # 系统托盘
│  │  └─ desktop/mod.rs          # ★ Win32：鼠标穿透 / 置底 / WorkerW 嵌入
│  ├─ tauri.conf.json            # 双窗口配置
│  └─ Cargo.toml
└─ scripts/gen-icons.mjs         # 生成图标（纯 Node）
```

---

## 环境要求（Windows 上构建）

> ⚠️ **必须在 Windows 10/11 上构建。** Tauri 的 Windows 包无法从 macOS/Linux 交叉编译，且桌面层的鼠标穿透 / WorkerW 嵌入是 Windows 专有能力，只能在 Windows 真机运行验证。

1. **Node.js** ≥ 18（建议 20+）
2. **Rust** 工具链：https://rustup.rs/ （安装后 `rustc --version` 可用）
3. **Microsoft C++ Build Tools**（含 MSVC）——Tauri 编译需要
4. **WebView2 Runtime**：Win11 自带；Win10 若无，安装包会引导安装

---

## 安装与运行

```bash
# 1. 安装前端依赖
npm install

# 2. 生成图标（首次或修改图标脚本后）
node scripts/gen-icons.mjs

# 3. 开发模式（热更新，启动主界面 + 桌面层）
npm run tauri:dev

# 4. 构建安装包（输出 NSIS 安装程序）
npm run tauri:build
# 产物：src-tauri/target/release/bundle/nsis/MyToDo_1.0.0_x64-setup.exe
```

### 仅预览界面（任意平台，含 macOS）

不装 Rust 也能在浏览器里预览 UI 与交互逻辑（数据走 localStorage mock）：

```bash
npm run dev
# 主界面: http://localhost:1420/
# 桌面层: http://localhost:1420/desktop.html
```

> mock 模式下：托盘、鼠标穿透、置底、开机自启等 Windows 能力为空操作；其余增删改查、过滤、搜索、设置、桌面层自适应布局均可正常体验。

---

## 桌面层实现说明（src-tauri/src/desktop/mod.rs）

- **鼠标穿透（锁定态）**：对窗口设置 `WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_NOACTIVATE`，点击/拖拽穿透到桌面。
- **解锁态**：去掉穿透与 NOACTIVATE，窗口可被拖拽移动、边缘缩放，显示极细边框（前端绘制）。
- **始终置底**：`SetWindowPos(HWND_BOTTOM)`；并由后台线程每 3 秒重新置底，抵御"显示桌面"/资源管理器重启导致的失效。
- **壁纸层嵌入（尽力）**：向 `Progman` 发送 `0x052C` 消息生成 `WorkerW`，再 `SetParent` 把窗口挂到壁纸层。失败则自动退化为"始终置底"。
- **位置/大小记忆**：窗口移动/缩放时写入数据文件，下次启动还原。

> 这些行为依赖具体 Windows 版本与显卡驱动表现，请在 Win10、Win11 上分别验证；WorkerW 嵌入在部分多显示器/特定壁纸引擎下可能回退到置底模式（属预期降级）。

---

## 数据存储

事项与设置以 JSON 保存在用户 AppData 目录：
`%APPDATA%\top.dqit.mytodo.app\mytodo-data.json`
采用原子写（先写 `.tmp` 再替换）；文件损坏时自动备份为 `.bak` 并回退默认值。
