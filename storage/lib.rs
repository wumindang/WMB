use std::fs;  // 中文注释：引入文件系统模块
use std::path::Path;  // 中文注释：引入路径处理模块

// 中文注释：定义存储结构，包含文件路径
pub struct Storage {
    path: String,  // 中文注释：存储文件路径
}

impl Storage {
    // 中文注释：创建新的存储实例，指定文件路径
    pub fn new(path: &str) -> Self {
        Storage { path: path.to_string() }  // 中文注释：初始化存储路径
    }

    // 中文注释：将数据保存到文件中
    pub fn save(&self, key: &str, value: &[u8]) -> std::io::Result<()> {
        let full_path = format!("{}/{}", self.path, key);  // 中文注释：构造完整文件路径
        if let Some(parent) = Path::new(&full_path).parent() {
            fs::create_dir_all(parent)?;  // 中文注释：创建父目录
        }
        fs::write(&full_path, value)?;  // 中文注释：写入数据到文件
        Ok(())  // 中文注释：返回成功
    }

    // 中文注释：从文件中加载数据
    pub fn load(&self, key: &str) -> std::io::Result<Option<Vec<u8>>> {
        let full_path = format!("{}/{}", self.path, key);  // 中文注释：构造完整文件路径
        match fs::read(&full_path) {
            Ok(data) => Ok(Some(data)),  // 中文注释：成功读取返回数据
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),  // 中文注释：文件不存在返回 None
            Err(e) => Err(e),  // 中文注释：其他错误返回错误
        }
    }
}

// 中文注释：测试模块，用于验证存储层功能
#[cfg(test)]
mod tests {
    use super::*;

    // 中文注释：测试存储和加载功能
    #[test]
    fn test_storage() {
        let storage = Storage::new("test_storage");  // 中文注释：创建测试存储实例
        let key = "test_key";
        let value = b"test_value";

        storage.save(key, value).unwrap();  // 中文注释：保存数据
        let loaded = storage.load(key).unwrap();  // 中文注释：加载数据
        assert_eq!(loaded, Some(value.to_vec()), "中文注释：验证加载的数据");  // 中文注释：检查结果

        fs::remove_file(format!("test_storage/{}", key)).unwrap();  // 中文注释：清理测试文件
        fs::remove_dir("test_storage").unwrap();  // 中文注释：清理测试目录
    }
}