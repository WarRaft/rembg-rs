use image::{RgbImage, RgbaImage};

pub struct RemovalResult {
    pub image: RgbaImage,
    pub mask: RgbImage,
}

impl RemovalResult {
    pub fn image(&self) -> &RgbaImage {
        &self.image
    }

    pub fn mask(&self) -> &RgbImage {
        &self.mask
    }

    pub fn into_parts(self) -> (RgbaImage, RgbImage) {
        (self.image, self.mask)
    }
}
