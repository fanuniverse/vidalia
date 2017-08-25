#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[serde(default)]
    pub transforms: Vec<Transform>
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum Transform {
    #[serde(rename = "downsize")]
    Downsize {
        name: String,
        width: usize
    },
    #[serde(rename = "webm")]
    WebM
}

pub struct ProcessingResult {
    pub analyzed: AnalyzedImage,
    pub transformed: Vec<TransformedImage>
}

pub struct TransformedImage {
    pub name: String,
    pub blob: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalyzedImage {
    pub width: usize,
    pub height: usize,
    pub hash: String
}
