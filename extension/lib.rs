use core::{Chain, Block};  // 中文注释：引入核心层功能
use storage::Storage;  // 中文注释：引入存储层
use interface::Interface;  // 中文注释：引入接口层
use serde::{Serialize, Deserialize};  // 中文注释：引入序列化支持

// 中文注释：定义扩展层的插件结构
#[derive(Serialize, Deserialize, Debug)]
pub struct Plugin {
    name: String,      // 中文注释：插件名称
    version: String,   // 中文注释：插件版本
}

// 中文注释：扩展层主结构，提供插件功能
pub struct Extension {
    interface: Interface,  // 中文注释：接口层实例
    plugins: Vec<Plugin>,  // 中文注释：插件列表
}

impl Extension {
    // 中文注释：创建新的扩展实例
    pub fn new(storage_path: &str, network_addr: &str) -> Self {
        let interface = Interface::new(storage_path, network_addr);  // 中文注释：初始化接口
        Extension { interface, plugins: Vec::new() }  // 中文注释：初始化插件列表
    }

    // 中文注释：注册新插件
    pub fn register_plugin(&mut self, name: &str, version: &str) {
        let plugin = Plugin {
            name: name.to_string(),      // 中文注释：设置插件名称
            version: version.to_string(),  // 中文注释：设置插件版本
        };
        self.plugins.push(plugin);  // 中文注释：添加插件到列表
        println!("中文注释：插件已注册: {} v{}", name, version);
    }

    // 中文注释：获取插件数量
    pub fn get_plugin_count(&self) -> usize {
        self.plugins.len()  // 中文注释：返回插件数量
    }

    // 中文注释：通过扩展层访问区块链区块数量
    pub fn get_block_count(&self) -> usize {
        self.interface.get_block_count()  // 中文注释：调用接口层方法
    }
}

// 中文注释：测试模块，用于验证扩展层功能
#[cfg(test)]
mod tests {
    use super::*;

    // 中文注释：测试插件注册功能
    #[test]
    fn test_register_plugin() {
        let mut extension = Extension::new("test_ext.dat", "127.0.0.1:8090");  // 中文注释：创建扩展实例
        extension.register_plugin("日志插件", "1.0.0");  // 中文注释：注册插件
        assert_eq!(extension.get_plugin_count(), 1, "中文注释：验证插件数量");  // 中文注释：检查数量

        let plugin = &extension.plugins[0];
        assert_eq!(plugin.name, "日志插件", "中文注释：验证插件名称");  // 中文注释：检查名称
        assert_eq!(plugin.version, "1.0.0", "中文注释：验证插件版本");  // 中文注释：检查版本
    }

    // 中文注释：测试区块链集成
    #[test]
    fn test_block_count() {
        let extension = Extension::new("test_ext_block.dat", "127.0.0.1:8090");  // 中文注释：创建扩展实例
        assert_eq!(extension.get_block_count(), 0, "中文注释：验证初始区块数量");  // 中文注释：检查初始数量
    }
}