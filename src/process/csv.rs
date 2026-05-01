use crate::OutputFormat;
use anyhow::{Context, Result};
use csv::ReaderBuilder;

pub fn process_csv(
    input: &str,
    with_header: bool,
    delimiter: char,
    output: String,
    format: OutputFormat,
) -> Result<()> {
    validate_delimiter(delimiter)?;

    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(with_header)
        .from_path(input)
        .with_context(|| format!("failed to open file {}", input))?;

    let mut result_data = Vec::with_capacity(128);

    if with_header {
        let headers = reader
            .headers()
            .with_context(|| "failed to read header")?
            .clone();
        for record_result in reader.records() {
            let record = record_result.with_context(|| "failed to read raw record")?;
            let json_value = headers
                .iter()
                .zip(record.iter())
                .collect::<serde_json::Value>();
            result_data.push(json_value);
        }
    } else {
        for record_result in reader.records() {
            let record = record_result.with_context(|| "failed to read record")?;
            let json_value = record.iter().collect::<serde_json::Value>();
            result_data.push(json_value);
        }
    }

    let output_content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&result_data)
            .with_context(|| "failed to convert to json")?,
        OutputFormat::Yaml => {
            yaml_serde::to_string(&result_data).with_context(|| "failed to convert to yaml")?
        }
    };
    std::fs::write(&output, output_content)
        .with_context(|| format!("failed to write to file {}", output))?;

    Ok(())
}

fn validate_delimiter(delimiter: char) -> Result<()> {
    if delimiter == '\0' {
        return Err(anyhow::anyhow!("delimiter cannot be null character"));
    }
    if delimiter.len_utf8() > 1 {
        return Err(anyhow::anyhow!("delimiter must be a single-byte character"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_process_csv_to_json() -> Result<()> {
        let mut input_file = NamedTempFile::new()?;
        let csv_content = r#"name,age,city
Alice,30,New York
Bob,25,London
"#;
        input_file.write_all(csv_content.as_bytes())?;
        let input_path = input_file.path().to_str().unwrap();
        let output_file = NamedTempFile::new()?;
        let output_path = output_file.path().to_str().unwrap().to_string();

        process_csv(
            input_path,
            true,
            ',',
            output_path.clone(),
            OutputFormat::Json,
        )?;

        let file_content = std::fs::read_to_string(output_path)?;

        assert!(file_content.contains("Alice"));
        assert!(file_content.contains("New York"));
        assert!(file_content.contains("25"));

        let json: serde_json::Value = serde_json::from_str(&file_content)?;
        assert_eq!(json.as_array().unwrap().len(), 2);

        Ok(())
    }

    #[test]
    fn test_process_csv_to_yaml() -> Result<()> {
        let mut input_file = NamedTempFile::new()?;
        let csv_content = "id,skill\n1,Rust\n2,Testing\n";
        input_file.write_all(csv_content.as_bytes())?;
        let input_path = input_file.path().to_str().unwrap();
        let output_file = NamedTempFile::new()?;
        let output_path = output_file.path().to_str().unwrap().to_string();

        process_csv(
            input_path,
            true,
            ',',
            output_path.clone(),
            OutputFormat::Yaml,
        )?;

        let file_content = std::fs::read_to_string(output_path)?;
        assert!(file_content.contains("Rust"));
        assert!(file_content.contains("Testing"));

        let yaml: yaml_serde::Value = yaml_serde::from_str(&file_content)?;
        assert!(yaml.is_sequence());

        Ok(())
    }

    #[test]
    fn test_csv_file_not_found() {
        let res = process_csv(
            "not_exist.csv",
            true,
            ',',
            "output.json".to_string(),
            OutputFormat::Json,
        );
        assert!(res.is_err());
    }
}
