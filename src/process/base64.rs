use crate::cli::Base64Format;
use anyhow::Context;
use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use std::fs::File;
use std::io::{BufReader, Read, stdin};

/// Base64编码（支持标准/URL安全模式）
/// - input: 输入文件路径【"-"表示标准输入】
/// - format: 编码格式 [Base64Format]
pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut buffer = Vec::new();
    get_reader(input)?.read_to_end(&mut buffer)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buffer),
    };
    Ok(encoded)
}

/// Base64解码（支持标准/URL安全模式）
/// - input: 输入文件路径【"-"表示标准输入】
/// - format: 解码格式 [Base64Format]
pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut buffer = String::new();
    get_reader(input)?.read_to_string(&mut buffer)?;

    let buffer = buffer.trim(); // avoid accidental newlines
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer),
    }?;
    Ok(decoded)
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(BufReader::new(stdin()))
    } else {
        Box::new(File::open(input).with_context(|| format!("failed to open {}", input))?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Base64Format;
    use std::io::Write;

    // helper: write bytes to a temp file, return its path as String
    fn write_temp(content: &[u8]) -> (tempfile::NamedTempFile, String) {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(content).unwrap();
        let path = file.path().to_str().unwrap().to_string();
        (file, path) // return file too — dropping it deletes it
    }

    #[test]
    fn test_encode_standard() {
        let (_file, path) = write_temp(b"hello world");
        let result = process_base64_encode(&path, Base64Format::Standard).unwrap();
        assert_eq!(result, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_encode_urlsafe() {
        let (_file, path) = write_temp(b"\xfb\xff\xfe");
        let result = process_base64_encode(&path, Base64Format::UrlSafe).unwrap();
        // URL-safe replaces '+' with '-' and '/' with '_', no padding
        assert_eq!(result, "-__-");
    }

    #[test]
    fn test_encode_empty() {
        let (_file, path) = write_temp(b"");
        let result = process_base64_encode(&path, Base64Format::Standard).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_decode_standard() {
        let (_file, path) = write_temp(b"aGVsbG8gd29ybGQ=");
        let result = process_base64_decode(&path, Base64Format::Standard).unwrap();
        assert_eq!(result, b"hello world");
    }

    #[test]
    fn test_decode_urlsafe() {
        let (_file, path) = write_temp(b"-__-");
        let result = process_base64_decode(&path, Base64Format::UrlSafe).unwrap();
        assert_eq!(result, b"\xfb\xff\xfe");
    }

    #[test]
    fn test_decode_trims_trailing_newline() {
        // files often have a trailing newline — your .trim() must handle it
        let (_file, path) = write_temp(b"aGVsbG8gd29ybGQ=\n");
        let result = process_base64_decode(&path, Base64Format::Standard).unwrap();
        assert_eq!(result, b"hello world");
    }

    #[test]
    fn test_decode_binary() {
        // binary round-trip: all 256 byte values must survive encode → decode
        let original: Vec<u8> = (0u8..=255).collect();
        let encoded = STANDARD.encode(&original);

        let (_file, path) = write_temp(encoded.as_bytes());
        let result = process_base64_decode(&path, Base64Format::Standard).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn test_roundtrip_standard() {
        let original = b"round trip test 123 !@#";
        let (_file, path) = write_temp(original);
        let encoded = process_base64_encode(&path, Base64Format::Standard).unwrap();

        let (_file2, path2) = write_temp(encoded.as_bytes());
        let decoded = process_base64_decode(&path2, Base64Format::Standard).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_roundtrip_urlsafe() {
        let original = b"url safe round trip ~!@#$%";
        let (_file, path) = write_temp(original);
        let encoded = process_base64_encode(&path, Base64Format::UrlSafe).unwrap();

        let (_file2, path2) = write_temp(encoded.as_bytes());
        let decoded = process_base64_decode(&path2, Base64Format::UrlSafe).unwrap();
        assert_eq!(decoded, original);
    }

    // ── error cases ──
    #[test]
    fn test_encode_missing_file() {
        let result = process_base64_encode("/nonexistent/file.txt", Base64Format::Standard);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("failed to open"));
    }

    #[test]
    fn test_decode_invalid_base64() {
        let (_file, path) = write_temp(b"not!valid==");
        let result = process_base64_decode(&path, Base64Format::Standard);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_wrong_format() {
        // standard-encoded input decoded as URL-safe must fail
        let (_file, path) = write_temp(b"+//+"); // valid standard, invalid URL-safe
        let result = process_base64_decode(&path, Base64Format::UrlSafe);
        assert!(result.is_err());
    }
}
