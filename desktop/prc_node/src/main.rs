use tauri::Manager;  // 中文注释：引入 Tauri 管理器，用于管理应用窗口和事件
use interface::Interface;  // 中文注释：引入接口层，提供区块链功能

// 中文注释：启动省储备委员会节点命令，前端通过 Tauri 调用此函数
#[tauri::command]
async fn start_node() {
    println!("中文注释：省储备委员会节点启动");  // 中文注释：在终端打印启动信息
    // 中文注释：实际节点逻辑待实现，例如初始化网络连接
}

// 中文注释：获取省储备委员会节点状态命令，前端通过 Tauri 调用此函数
#[tauri::command]
async fn get_status() -> usize {
    let interface = Interface::new("prc_node.dat", "127.0.0.1:8082");  // 中文注释：初始化接口实例，连接存储和网络
    interface.get_block_count()  // 中文注释：返回当前区块链的区块数量
}

fn main() {
    tauri::Builder::default()  // 中文注释：创建 Tauri 应用实例
        .invoke_handler(tauri::generate_handler![start_node, get_status])  // 中文注释：注册前端可调用的 Rust 函数
        .run(tauri::generate_context!())  // 中文注释：运行 Tauri 应用，使用配置文件
        .expect("中文注释：Tauri 应用启动失败");  // 中文注释：启动失败时抛出错误
}