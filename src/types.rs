#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[serde(default = "default_analyzers")]
    pub analyzers: Vec<String>,

    #[serde(default = "default_transforms")]
    pub transforms: Vec<Transform>
}

fn default_analyzers() -> Vec<String> { vec![] }
fn default_transforms() -> Vec<Transform> { vec![] }

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
    pub analyzed: Option<AnalyzedImage>,
    pub transformed: Vec<TransformedImage>
}

pub struct TransformedImage {
    pub name: String,
    pub blob: Vec<u8>
}

#[derive(Deserialize, Debug)]
pub struct AnalyzedImage {
    pub width: Option<usize>
}
