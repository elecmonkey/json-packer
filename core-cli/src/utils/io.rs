use std::fs;
use std::io::{Read, Write, stdin, stdout};
use std::path::Path;
use crate::error::{CliError, Result};

/// 从文件或stdin读取内容
pub fn read_input(input: &str) -> Result<String> {
    if input == "-" {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        let path = Path::new(input);
        if !path.exists() {
            return Err(CliError::FileNotFound(input.to_string()));
        }
        Ok(fs::read_to_string(path)?)
    }
}

/// 从文件或stdin读取字节数据
pub fn read_input_bytes(input: &str) -> Result<Vec<u8>> {
    if input == "-" {
        let mut buffer = Vec::new();
        stdin().read_to_end(&mut buffer)?;
        Ok(buffer)
    } else {
        let path = Path::new(input);
        if !path.exists() {
            return Err(CliError::FileNotFound(input.to_string()));
        }
        Ok(fs::read(path)?)
    }
}

/// 写入内容到文件或stdout
pub fn write_output(output: Option<&str>, content: &str) -> Result<()> {
    match output {
        Some(path) => {
            fs::write(path, content)?;
        }
        None => {
            stdout().write_all(content.as_bytes())?;
            stdout().flush()?;
        }
    }
    Ok(())
}

/// 写入字节数据到文件或stdout
pub fn write_output_bytes(output: Option<&str>, data: &[u8]) -> Result<()> {
    match output {
        Some(path) => {
            fs::write(path, data)?;
        }
        None => {
            stdout().write_all(data)?;
            stdout().flush()?;
        }
    }
    Ok(())
}

