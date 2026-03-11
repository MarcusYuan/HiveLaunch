//! HiveLaunch 移动端入口
//!
//! ## 架构说明
//!
//! 移动端是「纯前端客户端」，所有实际执行在云端完成：
//!
//! ```
//! ┌─────────────────────────────────────┐
//! │  移动端 App (本代码)                 │
//! │  - WebView 容器                     │
//! │  - 系统插件（通知等）                │
//! │  - 无后端逻辑                        │
//! └──────────────┬──────────────────────┘
//!                │ HTTPS
//!                ▼
//! ┌─────────────────────────────────────┐
//! │  云端后端                           │
//! │  - Agent 执行 (opencode/claude)    │
//! │  - Git 操作                         │
//! │  - 数据库                           │
//! └─────────────────────────────────────┘
//! ```
//!
//! ## 不包含的功能
//!
//! - ❌ AgentProcessManager（Agent 在云端执行）
//! - ❌ HTTP Server（不需要本地服务）
//! - ❌ Git 命令处理（Git 在云端执行）
//! - ❌ SQLite 数据库（数据在云端）
//! - ❌ opencode/claude CLI（Agent 在云端运行）


/// 移动端入口函数
///
/// 仅初始化 WebView 容器和必要的系统插件
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 添加调试日志
    println!("HiveLaunch Mobile: Starting initialization...");

    let context = tauri::generate_context!();
    println!("HiveLaunch Mobile: Context generated");

    tauri::Builder::default()
        // 系统插件
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        // 设置日志
        .setup(|_app| {
            println!("HiveLaunch Mobile: Setup called");
            Ok(())
        })
        // 运行应用
        .run(context)
        .expect("error while running tauri application");
}
