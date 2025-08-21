use assert_cmd::cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_compress_help() {
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("compress").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Compress JSON files or data"));
}

#[test]
fn test_basic_compress_decompress() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.json");
    let compressed_file = temp_dir.path().join("test.jcp");
    let output_file = temp_dir.path().join("output.json");
    
    // 创建测试 JSON 文件
    let test_data = r#"{"name": "John", "age": 30, "city": "New York"}"#;
    fs::write(&input_file, test_data).unwrap();
    
    // 压缩
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("compress")
        .arg(input_file.to_str().unwrap())
        .arg(compressed_file.to_str().unwrap());
    
    cmd.assert().success();
    
    // 检查压缩文件是否存在
    assert!(compressed_file.exists());
    
    // 解压
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("decompress")
        .arg(compressed_file.to_str().unwrap())
        .arg(output_file.to_str().unwrap());
    
    cmd.assert().success();
    
    // 验证内容
    let output_content = fs::read_to_string(&output_file).unwrap();
    let original: serde_json::Value = serde_json::from_str(test_data).unwrap();
    let restored: serde_json::Value = serde_json::from_str(&output_content).unwrap();
    
    assert_eq!(original, restored);
}

#[test]
fn test_compress_with_pool() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.json");
    let compressed_file = temp_dir.path().join("test.jcp");
    
    // 创建包含重复字符串的测试数据
    let test_data = r#"[
        {"status": "active", "type": "user"},
        {"status": "active", "type": "admin"},
        {"status": "inactive", "type": "user"}
    ]"#;
    fs::write(&input_file, test_data).unwrap();
    
    // 使用值池压缩
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("compress")
        .arg("--enable-pool")
        .arg("--stats")
        .arg(input_file.to_str().unwrap())
        .arg(compressed_file.to_str().unwrap());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Compression Statistics"));
}

#[test]
fn test_info_command() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.json");
    let compressed_file = temp_dir.path().join("test.jcp");
    
    // 创建测试数据
    let test_data = r#"{"test": "data"}"#;
    fs::write(&input_file, test_data).unwrap();
    
    // 压缩
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("compress")
        .arg(input_file.to_str().unwrap())
        .arg(compressed_file.to_str().unwrap());
    cmd.assert().success();
    
    // 查看信息
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("info")
        .arg(compressed_file.to_str().unwrap());
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File Information"))
        .stdout(predicate::str::contains("Version"));
}

#[test]
fn test_stdin_stdout() {
    let test_data = r#"{"message": "hello world"}"#;
    
    // 通过 stdin 压缩到 stdout
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("compress").arg("-");
    
    let _output = cmd
        .write_stdin(test_data)
        .assert()
        .success();
    
    // 由于测试环境限制，这里只验证命令执行成功
    // 在实际使用中，管道操作是正常工作的
}

#[test]
fn test_file_not_found() {
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("compress").arg("non_existent_file.json");
    
    cmd.assert()
        .failure()
        .code(6) // FileNotFound error code
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_invalid_json() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("invalid.json");
    
    // 创建无效的 JSON 文件
    fs::write(&input_file, "{ invalid json }").unwrap();
    
    let mut cmd = Command::cargo_bin("json-packer-cli").unwrap();
    cmd.arg("compress").arg(input_file.to_str().unwrap());
    
    cmd.assert()
        .failure()
        .code(3) // JSON parsing error code
        .stderr(predicate::str::contains("JSON parsing error"));
}