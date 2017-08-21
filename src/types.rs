#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub transforms: Vec<Transform>
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum Transform {
    #[serde(rename = "downsize")]
    Downsize {
        width: usize
    },
    #[serde(rename = "webm")]
    WebM
}

pub struct ProcessingResult {
    pub image: Vec<u8>
}
