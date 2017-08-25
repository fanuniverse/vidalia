use img_hash::{ImageHash, HashImage, HashType};
use magick_rust::{MagickWand, FilterType, ColorspaceType};

#[derive(Debug)]
pub struct MagickHashImage<'a> {
    wand: MagickWand,
    format: &'a str,
    dimensions: (u32, u32)
}

impl<'a> MagickHashImage<'a> {
    pub fn new(blob: &Vec<u8>, format: &'a str, width: usize, height: usize)
            -> Result<MagickHashImage<'a>, &'static str> {
        let mut wand = MagickWand::new();
        wand.read_image_blob(blob)?;

        if wand.get_image_format()? == "GIF" {
            wand.set_iterator_index(0)?;
        }

        Ok(MagickHashImage {
            wand: wand,
            format: format,
            dimensions: (width as u32, height as u32)
        })
    }

    pub fn dct_hash(&self) -> String {
        let hash = ImageHash::hash(self, 8, HashType::DCT);
        format!("{:?}", hash.bitv)
    }
}

impl<'a> HashImage for MagickHashImage<'a> {
    type Grayscale = Self;

    fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    fn resize(&self, width: u32, height: u32) -> Self {
        let resized_wand = self.wand.clone();
        resized_wand.resize_image(width as usize, height as usize, FilterType::LanczosFilter);

        Self {
            wand: resized_wand,
            format: self.format,
            dimensions: (width, height)
        }
    }

    fn grayscale(&self) -> Self {
        let grayscale_wand = self.wand.clone();
        grayscale_wand.transform_image_colorspace(ColorspaceType::GRAYColorspace);

        Self {
            wand: grayscale_wand,
            format: self.format,
            dimensions: self.dimensions
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        self.wand.export_image_pixels(0, 0,
            self.dimensions.0 as usize, self.dimensions.1 as usize, "I").unwrap()
    }

    /* The following methods are not used for DCT hashing. */

    fn channel_count() -> u8 {
        panic!("channel_count is not implemented")
    }

    fn foreach_pixel<F>(&self, _: F) where F: FnMut(u32, u32, &[u8]) {
        panic!("foreach_pixel is not implemented")
    }
}
