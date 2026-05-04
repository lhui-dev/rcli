use anyhow::{Context, Result};
use rand::prelude::*;
use std::io::Write;

const UPPER: &[u8] = b"ABCDEFGHIJKMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

/// 批量生成密码
/// - length: 密码长度
/// - uppercase: 是否包含大写字母:[A-Z]
/// - lowercase: 是否包含小写字母:[a-z]
/// - numeric: 是否包含数字:[1-9]
/// - symbolic: 是否包含符号
/// - batch_count: 批量生成数量，默认[1]
/// - output: 输出到文件或控制台
pub fn process_gen_passwd(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numeric: bool,
    symbolic: bool,
    batch_count: u32,
    output: Option<&str>,
) -> Result<()> {
    let mut passwd_vec = Vec::with_capacity(batch_count as usize);

    for _ in 0..batch_count {
        let pwd = gen_single_passwd(length, uppercase, lowercase, numeric, symbolic)?;
        passwd_vec.push(pwd);
    }

    match output {
        Some(output_path) => {
            let path = std::path::Path::new(output_path);

            let mut f = std::fs::File::create(path)
                .with_context(|| format!("Failed to create file {:?}", path))?;

            for passwd in passwd_vec {
                writeln!(f, "{}", passwd)
                    .with_context(|| format!("Failed to write passwd to file {:?}", path))?;
            }
        }
        None => {
            println!("Generated {} passwords:", batch_count);
            for (i, pwd) in passwd_vec.iter().enumerate() {
                println!("[{}] {}", i + 1, pwd);
            }
        }
    }

    Ok(())
}

fn gen_single_passwd(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numeric: bool,
    symbolic: bool,
) -> Result<String> {
    let mut passwd = Vec::new();
    let mut chars = Vec::new();

    // Available on crate feature thread_rng only.
    // See: https://docs.rs/rand/latest/rand/fn.rng.html
    let mut rng = rand::rng();

    if uppercase {
        chars.extend_from_slice(UPPER);
        passwd.push(
            *UPPER
                .choose(&mut rng)
                .with_context(|| "UPPER character set is empty (invalid configuration)")?,
        );
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
        passwd.push(
            *LOWER
                .choose(&mut rng)
                .with_context(|| "LOWER character set is empty (invalid configuration)")?,
        );
    }
    if numeric {
        chars.extend_from_slice(NUMBER);
        passwd.push(
            *NUMBER
                .choose(&mut rng)
                .with_context(|| "NUMBER character set is empty (invalid configuration)")?,
        );
    }
    if symbolic {
        chars.extend_from_slice(SYMBOL);
        passwd.push(
            *SYMBOL
                .choose(&mut rng)
                .with_context(|| "SYMBOL character set is empty (invalid configuration)")?,
        );
    }
    if chars.is_empty() {
        anyhow::bail!("No character sets available for password generation");
    }

    let remaining = length - passwd.len() as u8;
    for _ in 0..remaining {
        let c = chars
            .choose(&mut rng)
            .with_context(|| "No characters available to fill password (invalid configuration)")?;
        passwd.push(*c);
    }

    // 打乱顺序
    passwd.shuffle(&mut rng);
    let ret = String::from_utf8(passwd)
        .with_context(|| "Failed to convert password bytes to UTF-8 (invalid characters)")?;

    Ok(ret)
}
