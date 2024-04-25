use crate::cli::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use std::{fs::File, io, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    // 将 reader 中的所有数据读取到 buf 中，直到 reader 返回 EOF（文件结束）。
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    // read_to_string 方法用于将读取的数据存储为 String。
    reader.read_to_string(&mut buf)?;
    // trim() 方法用于去除字符串两端的空白字符，包括空格、制表符和换行符。
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decode = String::from_utf8(decode)?;
    println!("{}", decode);
    Ok(())
}

// 为什么要使用 Box<dyn Read> 类型？
// 使用 Box<dyn Read> 使得函数能够统一这两种不同来源的返回类型，提供了很高的灵活性。
// 由于标准输入和文件输入都实现了 Read trait，我们可以使用 trait 对象来统一这两种类型。
fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}
