use crate::Block;  // 中文注释：引入 Block 结构体
use crate::wmb_storage::Storage;  // 中文注释：引入存储层

// 中文注释：定义区块结构，包含 ID 和数据
pub struct Block {
    pub id: u32,       // 中文注释：区块编号
    pub data: String,  // 中文注释：区块数据
}

impl Block {
    // 中文注释：创建新区块，传入编号和数据
    pub fn new(id: u32, data: String) -> Self {
        Block { id, data }  // 中文注释：返回新创建的区块实例
    }

    // 中文注释：将区块保存到存储层
    pub fn save(&self, storage: &Storage) -> std::io::Result<()> {
        let value = format!("{}-{}", self.id, self.data);  // 中文注释：将区块数据转换为字符串
        storage.save(&self.id.to_string(), value.as_bytes())  // 中文注释：保存到存储层
    }
}