// 中文注释：SQLite 存储模块，为所有节点提供本地存储功能（归档节点存完整数据，非归档节点存区块头）
pub mod sqlite {
    use core::Core;  // 中文注释：引入核心层模块（尽管当前未直接使用）
    use rusqlite::{Connection, Result as SqliteResult};  // 中文注释：引入 SQLite 依赖
    use std::path::Path;
    use std::error::Error;

    // 中文注释：SQLite 存储客户端结构体
    pub struct SqliteClient {
        conn: Connection,  // 中文注释：SQLite 数据库连接
    }

    impl SqliteClient {
        // 中文注释：创建 SQLite 客户端实例，初始化数据库
        pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
            let conn = Connection::open(Path::new(db_path))?;  // 中文注释：打开或创建数据库文件
            Self::init_tables(&conn)?;  // 中文注释：初始化表结构
            Ok(SqliteClient { conn })
        }

        // 中文注释：初始化数据库表结构
        fn init_tables(conn: &Connection) -> SqliteResult<()> {
            // 中文注释：创建 blocks 表，存储完整区块（归档节点）
            conn.execute(
                "CREATE TABLE IF NOT EXISTS blocks (
                    height INTEGER PRIMARY KEY,  -- 中文注释：区块高度，主键
                    data TEXT NOT NULL           -- 中文注释：区块数据
                )",
                [],
            )?;

            // 中文注释：创建 headers 表，存储区块头（非归档节点）
            conn.execute(
                "CREATE TABLE IF NOT EXISTS headers (
                    height INTEGER PRIMARY KEY,  -- 中文注释：区块高度，主键
                    header TEXT NOT NULL         -- 中文注释：区块头数据
                )",
                [],
            )?;
            Ok(())
        }

        // 中文注释：存储完整区块（数据归档节点使用），返回高度
        pub fn store_block(&self, block: &str, height: u64) -> Result<(), Box<dyn Error>> {
            self.conn.execute(
                "INSERT OR REPLACE INTO blocks (height, data) VALUES (?1, ?2)",
                &[&height.to_string(), block],
            )?;
            Ok(())
        }

        // 中文注释：存储区块头（非数据归档节点使用），返回高度
        pub fn store_header(&self, header: &str, height: u64) -> Result<(), Box<dyn Error>> {
            self.conn.execute(
                "INSERT OR REPLACE INTO headers (height, header) VALUES (?1, ?2)",
                &[&height.to_string(), header],
            )?;
            Ok(())
        }

        // 中文注释：获取完整区块（数据归档节点使用）
        pub fn get_block(&self, height: u64) -> Result<String, Box<dyn Error>> {
            let block: String = self.conn.query_row(
                "SELECT data FROM blocks WHERE height = ?1",
                &[&height.to_string()],
                |row| row.get(0),
            )?;
            Ok(block)
        }

        // 中文注释：获取区块头（非数据归档节点使用）
        pub fn get_header(&self, height: u64) -> Result<String, Box<dyn Error>> {
            let header: String = self.conn.query_row(
                "SELECT header FROM headers WHERE height = ?1",
                &[&height.to_string()],
                |row| row.get(0),
            )?;
            Ok(header)
        }

        // 中文注释：备份数据库到文件
        pub fn backup(&self, backup_path: &str) -> Result<(), Box<dyn Error>> {
            let backup_conn = Connection::open(backup_path)?;  // 中文注释：打开备份文件
            self.conn.backup(rusqlite::DatabaseName::Main, &backup_conn, None)?;  // 中文注释：执行备份
            backup_conn.close().map_err(|(_, e)| Box::new(e) as Box<dyn Error>)?;
            Ok(())
        }
    }
}

// 中文注释：模块测试
#[cfg(test)]
mod tests {
    use super::sqlite::SqliteClient;

    #[test]
    fn test_store_and_get_block() {
        let client = SqliteClient::new(":memory:").unwrap();  // 中文注释：使用内存数据库测试
        let block = "测试区块数据";
        let height = 1;
        client.store_block(block, height).unwrap();
        let retrieved = client.get_block(height).unwrap();
        assert_eq!(block, retrieved);
    }

    #[test]
    fn test_store_and_get_header() {
        let client = SqliteClient::new(":memory:").unwrap();
        let header = "测试区块头";
        let height = 1;
        client.store_header(header, height).unwrap();
        let retrieved = client.get_header(height).unwrap();
        assert_eq!(header, retrieved);
    }

    #[test]
    fn test_backup() {
        let client = SqliteClient::new(":memory:").unwrap();
        let block = "备份测试数据";
        client.store_block(block, 1).unwrap();
        client.backup("test_backup.db").unwrap();
        let backup_client = SqliteClient::new("test_backup.db").unwrap();
        let retrieved = backup_client.get_block(1).unwrap();
        assert_eq!(block, retrieved);
    }
}