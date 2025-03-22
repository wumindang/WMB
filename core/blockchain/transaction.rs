use crate::{sha256_hash, sign, verify}; // 从根模块导入
use serde::{Serialize, Deserialize};
use secp256k1::{SecretKey, PublicKey};
use secp256k1::ecdsa::Signature; // 使用推荐的类型

/// 交易结构体，记录五民币的资金转移信息
#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,         // 发送者公钥地址
    pub receiver: String,       // 接收者公钥地址
    pub amount_yuan: f64,       // 交易金额（单位：元，小数点后两位）
    pub fee_yuan: f64,          // 交易手续费（单位：元，小数点后两位）
    pub tx_type: String,        // 交易类型（"链上"或"链下"）
    pub signature: Option<Signature>, // 交易签名，可选字段
    pub hash: String,           // 交易哈希
}

// 手动实现 Serialize 和 Deserialize，因为 Signature 不支持默认序列化
impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Transaction", 7)?;
        state.serialize_field("sender", &self.sender)?;
        state.serialize_field("receiver", &self.receiver)?;
        state.serialize_field("amount_yuan", &self.amount_yuan)?;
        state.serialize_field("fee_yuan", &self.fee_yuan)?;
        state.serialize_field("tx_type", &self.tx_type)?;
        state.serialize_field("signature", &self.signature.map(|sig| sig.to_string()))?;
        state.serialize_field("hash", &self.hash)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Deserializer};
        use std::str::FromStr;

        enum Field { Sender, Receiver, AmountYuan, FeeYuan, TxType, Signature, Hash }
        impl<'de> de::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> de::Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`sender`, `receiver`, `amount_yuan`, `fee_yuan`, `tx_type`, `signature`, or `hash`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "sender" => Ok(Field::Sender),
                            "receiver" => Ok(Field::Receiver),
                            "amount_yuan" => Ok(Field::AmountYuan),
                            "fee_yuan" => Ok(Field::FeeYuan),
                            "tx_type" => Ok(Field::TxType),
                            "signature" => Ok(Field::Signature),
                            "hash" => Ok(Field::Hash),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct TransactionVisitor;
        impl<'de> de::Visitor<'de> for TransactionVisitor {
            type Value = Transaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Transaction")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<Transaction, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                let sender = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let receiver = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let amount_yuan = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let fee_yuan = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let tx_type = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let signature_str: Option<String> = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let hash = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(6, &self))?;
                let signature = signature_str.map(|s| Signature::from_str(&s).unwrap());
                Ok(Transaction {
                    sender,
                    receiver,
                    amount_yuan,
                    fee_yuan,
                    tx_type,
                    signature,
                    hash,
                })
            }
            fn visit_map<V>(self, mut map: V) -> Result<Transaction, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut sender = None;
                let mut receiver = None;
                let mut amount_yuan = None;
                let mut fee_yuan = None;
                let mut tx_type = None;
                let mut signature = None;
                let mut hash = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Sender => sender = Some(map.next_value()?),
                        Field::Receiver => receiver = Some(map.next_value()?),
                        Field::AmountYuan => amount_yuan = Some(map.next_value()?),
                        Field::FeeYuan => fee_yuan = Some(map.next_value()?),
                        Field::TxType => tx_type = Some(map.next_value()?),
                        Field::Signature => {
                            if let Some(sig_str) = map.next_value::<Option<String>>()? {
                                signature = Some(Signature::from_str(&sig_str).map_err(de::Error::custom)?);
                            }
                        }
                        Field::Hash => hash = Some(map.next_value()?),
                    }
                }
                Ok(Transaction {
                    sender: sender.ok_or_else(|| de::Error::missing_field("sender"))?,
                    receiver: receiver.ok_or_else(|| de::Error::missing_field("receiver"))?,
                    amount_yuan: amount_yuan.ok_or_else(|| de::Error::missing_field("amount_yuan"))?,
                    fee_yuan: fee_yuan.ok_or_else(|| de::Error::missing_field("fee_yuan"))?,
                    tx_type: tx_type.ok_or_else(|| de::Error::missing_field("tx_type"))?,
                    signature,
                    hash: hash.ok_or_else(|| de::Error::missing_field("hash"))?,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["sender", "receiver", "amount_yuan", "fee_yuan", "tx_type", "signature", "hash"];
        deserializer.deserialize_struct("Transaction", FIELDS, TransactionVisitor)
    }
}

impl Transaction {
    /// 创建一笔新交易
    /// 参数：
    /// - sender: 发送者地址
    /// - receiver: 接收者地址
    /// - amount_yuan: 交易金额（单位：元）
    /// - fee_yuan: 交易手续费（单位：元）
    /// - tx_type: 交易类型（"链上"或"链下"）
    /// 返回：新创建的交易
    pub fn new(sender: String, receiver: String, amount_yuan: f64, fee_yuan: f64, tx_type: String) -> Self {
        if !Self::is_valid_amount(amount_yuan) || !Self::is_valid_amount(fee_yuan) {
            panic!("金额或手续费精度超过两位小数");
        }
        let mut tx = Transaction {
            sender,
            receiver,
            amount_yuan,
            fee_yuan,
            tx_type,
            signature: None,
            hash: String::new(),
        };
        tx.hash = tx.calculate_hash();
        tx
    }

    /// 检查金额是否符合精度要求（最多两位小数）
    /// 参数：
    /// - amount: 待检查的金额
    /// 返回：是否有效
    fn is_valid_amount(amount: f64) -> bool {
        let amount_str = format!("{:.2}", amount);
        amount == amount_str.parse::<f64>().unwrap()
    }

    /// 计算交易的哈希值
    /// 返回：交易的SHA256哈希值（16进制字符串）
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.sender, self.receiver, self.amount_yuan, self.fee_yuan, self.tx_type
        );
        sha256_hash(&data)
    }

    /// 对交易进行签名
    /// 参数：
    /// - secret_key: 发送者的私钥
    pub fn sign(&mut self, secret_key: &SecretKey) {
        self.signature = Some(sign(&self.hash, secret_key));
    }

    /// 验证交易签名
    /// 参数：
    /// - public_key: 发送者的公钥
    /// 返回：签名是否有效
    pub fn verify(&self, public_key: &PublicKey) -> bool {
        if let Some(ref sig) = self.signature {
            verify(&self.hash, sig, public_key)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_keypair; // 添加导入

    #[test]
    fn test_transaction_creation_and_sign() {
        let (sk, pk) = generate_keypair();
        let sender = hex::encode(pk.serialize());
        let mut tx = Transaction::new(sender.clone(), "receiver_address".to_string(), 10.50, 0.05, "链上".to_string());
        tx.sign(&sk);
        assert_eq!(tx.amount_yuan, 10.50);
        assert_eq!(tx.tx_type, "链上");
        assert!(tx.verify(&pk));
    }

    #[test]
    #[should_panic]
    fn test_invalid_amount_precision() {
        Transaction::new("sender".to_string(), "receiver".to_string(), 10.123, 0.05, "链上".to_string());
    }
}