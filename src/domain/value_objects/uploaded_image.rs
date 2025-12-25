use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedImage {
    #[serde(alias = "secure_url")]
    pub url: String,
    pub public_id: String,
}

impl UploadedImage {
    pub fn new(url: String, public_id: String) -> Self {
        Self { url, public_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadBase64Image {
    pub base64_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadImageOptions {
    pub folder: Option<String>,
    pub public_id: Option<String>,
    pub transformation: Option<String>,
}
