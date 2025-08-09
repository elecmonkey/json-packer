use std::collections::HashMap;

use crate::{bitstream::{BitReader, BitWriter}, Error};

#[derive(Debug, Clone)]
pub struct HuffmanCodec {
    // 编码映射：key -> (LSB-first code, bit_len)
    enc_map: HashMap<String, (u64, u8)>,
    // 解码用二叉树
    root: Box<Node>,
}

#[derive(Debug, Clone)]
enum Node {
    Internal { left: Box<Node>, right: Box<Node> },
    Leaf(String),
}

impl HuffmanCodec {
    pub fn from_frequencies(freq_map: &HashMap<String, u64>) -> Result<Self, Error> {
        // 收集符号并排序（字典序）确保确定性
        let mut symbols: Vec<(String, u64)> = freq_map
            .iter()
            .map(|(k, &f)| (k.clone(), f))
            .collect();
        symbols.sort_by(|a, b| a.0.cmp(&b.0));

        if symbols.is_empty() {
            // 空字典：允许构建一个空的解码器（解码时会失败）
            return Ok(HuffmanCodec { enc_map: HashMap::new(), root: Box::new(Node::Internal { left: Box::new(Node::Leaf(String::new())), right: Box::new(Node::Leaf(String::new())) }) });
        }

        // 特殊情况：只有一个符号，分配长度1的码字 "0"
        if symbols.len() == 1 {
            let key = symbols[0].0.clone();
            let mut enc_map = HashMap::new();
            // LSB-first: 单比特0
            enc_map.insert(key.clone(), (0, 1));
            let root = Box::new(Node::Internal {
                left: Box::new(Node::Leaf(key)),
                right: Box::new(Node::Leaf(String::new())), // 未使用分支
            });
            return Ok(HuffmanCodec { enc_map, root });
        }

        // 1) 通过普通 Huffman 构建 code lengths（叶子深度）
        let code_lengths = build_code_lengths(&symbols);

        // 2) Canonical 编码：按 (len, key lex) 排序，生成 MSB-first 码字
        let mut by_len: Vec<(usize, &str)> = symbols
            .iter()
            .enumerate()
            .map(|(i, (k, _))| (code_lengths[i], k.as_str()))
            .collect();
        by_len.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(b.1)));

        let max_len = by_len.iter().map(|(l, _)| *l).max().unwrap_or(1);
        let mut bl_count = vec![0usize; max_len + 1];
        for (l, _) in &by_len { bl_count[*l] += 1; }

        // 计算每个长度的起始码（MSB-first）
        let mut next_code = vec![0u32; max_len + 1];
        let mut code: u32 = 0;
        for bits in 1..=max_len {
            code = (code + bl_count[bits - 1] as u32) << 1;
            next_code[bits] = code;
        }

        // 3) 构建编码映射与解码树
        let mut enc_map: HashMap<String, (u64, u8)> = HashMap::with_capacity(by_len.len());
        let mut root = Node::Internal { left: Box::new(Node::Leaf(String::new())), right: Box::new(Node::Leaf(String::new())) };

        for (len, key) in by_len {
            if len == 0 { return Err(Error::HuffmanError); }
            let code_msb = next_code[len];
            next_code[len] += 1;

            // 将 MSB-first 码字反转成 LSB-first 存储，便于 BitWriter 低位优先写入
            let code_lsb = reverse_low_bits(code_msb as u64, len as u8);
            enc_map.insert(key.to_string(), (code_lsb, len as u8));

            // 在解码树中插入（按照 MSB-first 路径）
            insert_codeword(&mut root, key, code_msb, len as u8)?;
        }

        Ok(HuffmanCodec { enc_map, root: Box::new(root) })
    }

    pub fn write_key_code(&self, key: &str, writer: &mut BitWriter) -> Result<(), Error> {
        let (code_lsb, len) = self
            .enc_map
            .get(key)
            .copied()
            .ok_or(Error::HuffmanError)?;
        writer.write_bits(code_lsb, len as u32);
        Ok(())
    }

    pub fn decode_key(&self, reader: &mut BitReader) -> Result<String, Error> {
        // 逐位读取并下行
        let mut node = self.root.as_ref();
        loop {
            match node {
                Node::Leaf(key) => return Ok(key.clone()),
                Node::Internal { left, right } => {
                    let bit = reader.read_bits(1)? as u8;
                    node = if bit == 0 { left.as_ref() } else { right.as_ref() };
                }
            }
        }
    }

    pub fn try_get_code(&self, key: &str) -> Option<(u64, u8)> { self.enc_map.get(key).copied() }
}

fn reverse_low_bits(mut v: u64, bits: u8) -> u64 {
    let mut r = 0u64;
    for _ in 0..bits {
        r = (r << 1) | (v & 1);
        v >>= 1;
    }
    r
}

#[derive(Debug, Clone)]
struct HeapNode {
    freq: u64,
    // 为了确定性，包含字典序最小的叶子索引作为 tie-breaker
    min_sym_idx: usize,
    node: Box<TreeNode>,
}

#[derive(Debug, Clone)]
enum TreeNode {
    Leaf { sym_idx: usize },
    Internal { left: Box<TreeNode>, right: Box<TreeNode> },
}

fn build_code_lengths(symbols: &[(String, u64)]) -> Vec<usize> {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    // 小根堆：通过 Ord 反转实现
    #[derive(Debug)]
    struct OrdNode(HeapNode);
    impl PartialEq for OrdNode { fn eq(&self, other: &Self) -> bool { self.0.freq == other.0.freq && self.0.min_sym_idx == other.0.min_sym_idx } }
    impl Eq for OrdNode {}
    impl PartialOrd for OrdNode {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }
    impl Ord for OrdNode {
        fn cmp(&self, other: &Self) -> Ordering {
            // 反转为小根堆：频率小的优先，其次最小符号索引优先
            other.0.freq.cmp(&self.0.freq).then(other.0.min_sym_idx.cmp(&self.0.min_sym_idx))
        }
    }

    let mut heap: BinaryHeap<OrdNode> = BinaryHeap::new();
    for (i, (_k, f)) in symbols.iter().enumerate() {
        heap.push(OrdNode(HeapNode { freq: *f, min_sym_idx: i, node: Box::new(TreeNode::Leaf { sym_idx: i }) }));
    }

    if heap.len() == 1 {
        return vec![1];
    }

    while heap.len() > 1 {
        let OrdNode(a) = heap.pop().unwrap();
        let OrdNode(b) = heap.pop().unwrap();
        let min_sym_idx = a.min_sym_idx.min(b.min_sym_idx);
        let merged = HeapNode {
            freq: a.freq + b.freq,
            min_sym_idx,
            node: Box::new(TreeNode::Internal { left: a.node, right: b.node }),
        };
        heap.push(OrdNode(merged));
    }

    let root = heap.pop().unwrap().0.node;
    // 计算叶子深度
    let mut code_lengths = vec![0usize; symbols.len()];
    fn walk(node: &TreeNode, depth: usize, lens: &mut [usize]) {
        match node {
            TreeNode::Leaf { sym_idx } => lens[*sym_idx] = depth.max(1),
            TreeNode::Internal { left, right } => {
                walk(left, depth + 1, lens);
                walk(right, depth + 1, lens);
            }
        }
    }
    walk(&root, 0, &mut code_lengths);
    code_lengths
}

fn insert_codeword(root: &mut Node, key: &str, code_msb: u32, len: u8) -> Result<(), Error> {
    let mut node = root;
    for i in (0..len).rev() { // 从 MSB 到 LSB
        let bit = ((code_msb >> i) & 1) as u8;
        match node {
            Node::Internal { left, right } => {
                if bit == 0 {
                    if matches!(left.as_ref(), Node::Leaf(s) if s.is_empty()) {
                        // 继续向下
                    } else if matches!(left.as_ref(), Node::Internal { .. }) {
                        // ok
                    } else if let Node::Leaf(_) = left.as_ref() {
                        // 将叶子展开为内部节点
                        *left = Box::new(Node::Internal { left: Box::new(Node::Leaf(String::new())), right: Box::new(Node::Leaf(String::new())) });
                    }
                    node = left.as_mut();
                } else {
                    if matches!(right.as_ref(), Node::Leaf(s) if s.is_empty()) {
                        // 继续向下
                    } else if matches!(right.as_ref(), Node::Internal { .. }) {
                        // ok
                    } else if let Node::Leaf(_) = right.as_ref() {
                        *right = Box::new(Node::Internal { left: Box::new(Node::Leaf(String::new())), right: Box::new(Node::Leaf(String::new())) });
                    }
                    node = right.as_mut();
                }
            }
            Node::Leaf(_) => {
                // 展开叶子为内部节点
                *node = Node::Internal { left: Box::new(Node::Leaf(String::new())), right: Box::new(Node::Leaf(String::new())) };
                if let Node::Internal { left, right } = node {
                    node = if bit == 0 { left.as_mut() } else { right.as_mut() };
                }
            }
        }
    }
    // 最后位置写入叶子
    *node = Node::Leaf(key.to_string());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_huffman() {
        let mut freq = HashMap::new();
        freq.insert("name".to_string(), 2);
        freq.insert("age".to_string(), 1);
        let codec = HuffmanCodec::from_frequencies(&freq).unwrap();
        assert!(codec.try_get_code("name").is_some());
        assert!(codec.try_get_code("age").is_some());
    }

    #[test]
    fn encode_decode_roundtrip() {
        let mut freq = HashMap::new();
        freq.insert("name".to_string(), 2);
        freq.insert("age".to_string(), 1);
        let codec = HuffmanCodec::from_frequencies(&freq).unwrap();
        let mut w = BitWriter::new();
        codec.write_key_code("name", &mut w).unwrap();
        codec.write_key_code("age", &mut w).unwrap();
        let bytes = w.into_bytes();
        let mut r = BitReader::new(&bytes);
        let k1 = codec.decode_key(&mut r).unwrap();
        let k2 = codec.decode_key(&mut r).unwrap();
        assert_eq!(k1, "name");
        assert_eq!(k2, "age");
    }
}
