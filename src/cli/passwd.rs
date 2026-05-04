use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPasswdOpts {
    #[arg(
        short = 'l',
        long,
        default_value_t = 16,
        help = "Length of the generated password"
    )]
    pub length: u8,

    /// 不包含大写字母[A-Z]
    #[arg(
        long,
        default_value_t = false,
        help = "Exclude uppercase letters (A-Z)"
    )]
    pub no_uppercase: bool,

    /// 不包含小写字母[a-z]
    #[arg(
        long,
        default_value_t = false,
        help = "Exclude lowercase letters (a-z)"
    )]
    pub no_lowercase: bool,

    /// 不包含数字[1-9]
    #[arg(long, default_value_t = false, help = "Exclude numeric (1-9)")]
    pub no_numeric: bool,

    /// 不包含特殊字符[!@#$%^&*]
    #[arg(
        long,
        default_value_t = false,
        help = "Exclude special symbols (!@#$%^&*)"
    )]
    pub no_symbolic: bool,

    /// 批量生成密码数量，默认为[1]
    #[arg(
        short = 'n',
        long,
        default_value_t = 1,
        help = "Batch generate N passwords (default: 1)"
    )]
    pub batch_count: u32,

    /// 输出文件位置
    #[arg(long, help = "Export passwords to file (default: print to console)")]
    pub output: Option<String>,
}

impl GenPasswdOpts {
    pub fn validate(&self) -> anyhow::Result<()> {
        // length cannot be zero.
        if self.length == 0 {
            anyhow::bail!("Password length must be greater than 0");
        }
        let enabled_charsets = [
            !self.no_uppercase,
            !self.no_symbolic,
            !self.no_lowercase,
            !self.no_numeric,
        ]
        .iter()
        .filter(|&&enabled| enabled)
        .count() as u8;

        // 验证：至少启用一个字符集
        if enabled_charsets == 0 {
            anyhow::bail!(
                "At least one of character set must be enabled（uppercase/lowercase/numeric/symbolic）"
            );
        }

        //  验证：密码长度 ≥ 启用的字符集数量（每个启用的字符集至少出现1次）
        if self.length < enabled_charsets {
            anyhow::bail!("Length must be at least {enabled_charsets} character length");
        }

        // 批量生成数量至少为1
        if self.batch_count == 0 {
            anyhow::bail!("Batch count must be greater than 0");
        }

        if let Some(output) = &self.output {
            let path = std::path::Path::new(output);
            if let Some(parent) = path.parent()
                && !parent.exists()
            {
                anyhow::bail!("Output directory {:?} does not exist", parent);
            }

            if path.is_dir() {
                anyhow::bail!("Output directory {:?} is a directory, not a file", path);
            }
        }

        Ok(())
    }
}
