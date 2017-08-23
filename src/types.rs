#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[serde(default)]
    pub analyzers: Vec<String>,

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AnalyzedImage {
    #[serde(skip_serializing_if="Option::is_none")]
    pub width: Option<usize>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub height: Option<usize>
}
