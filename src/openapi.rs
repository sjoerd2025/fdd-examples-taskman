use serde_yaml::Value;

#[derive(thiserror::Error, Debug)]
pub enum OpenApiError {
    #[error("failed to read OpenAPI file: {0}")]
    Read(#[from] std::io::Error),
    #[error("failed to parse OpenAPI yaml: {0}")]
    Parse(#[from] serde_yaml::Error),
}

pub async fn load_openapi(path: &str) -> Result<Value, OpenApiError> {
    let contents = tokio::fs::read_to_string(path).await?;
    let parsed: Value = serde_yaml::from_str(&contents)?;
    Ok(parsed)
}
