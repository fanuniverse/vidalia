#[derive(Deserialize, Debug)]
pub struct Manifest {
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
    pub images: Vec<TransformedImage>
}

pub struct TransformedImage {
    pub name: String,
    pub blob: Vec<u8>
}
