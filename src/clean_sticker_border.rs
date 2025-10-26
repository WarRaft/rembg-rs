use image::{GrayImage, Luma, Rgba, RgbaImage};
use imageproc::contrast::{ThresholdType, threshold};
use imageproc::distance_transform::{Norm, distance_transform};

/// Cleans border by adding a soft black outline OUTSIDE only (≈6px, feather ≈1.5px)
/// and keeps sticker RGB/alpha intact. No half-transparent pixel on outer arcs.
pub fn clean_sticker_border(img: &RgbaImage) -> RgbaImage {
    let (w, h) = img.dimensions();

    // 1) Binary mask of the sticker (alpha >= 16 is inside)
    let alpha = GrayImage::from_fn(w, h, |x, y| Luma([img.get_pixel(x, y)[3]]));
    let mask = threshold(&alpha, 16, ThresholdType::Binary);

    // 2) Distance from OUTSIDE to nearest inside pixel (round metric for smooth arcs)
    //    distance_transform returns per-pixel distance in pixel units as u32.
    //    Use L2 to avoid chessboard artifacts on curves.
    let dist_out = distance_transform(&mask, Norm::L2);

    // Stroke params
    let stroke: f32 = 6.0; // full strength up to 6px
    let feather: f32 = 1.5; // soft falloff outside (6..7.5px)
    let max_outline_alpha: u8 = 255;

    #[inline]
    fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
        let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    // 3) Composite: draw black outline BEHIND the sticker only where mask==0
    let mut out = RgbaImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let top = img.get_pixel(x, y);
            let inside = mask.get_pixel(x, y)[0] != 0;

            // background/outline
            let mut br = 0f32;
            let mut bg = 0f32;
            let mut bb = 0f32;
            let mut ba = 0f32;

            if !inside {
                let d = dist_out.get_pixel(x, y)[0] as f32;
                // outline alpha profile: full inside [0, stroke], smooth fade in (stroke..stroke+feather)
                let a01 = if d <= stroke {
                    1.0
                } else {
                    1.0 - smoothstep(stroke, stroke + feather, d)
                };
                let a = (a01 * (max_outline_alpha as f32)).round().clamp(0.0, 255.0) as u8;

                // Kill tiny tails completely to avoid single half-transparent pixels outside.
                let a = if a < 3 { 0 } else { a };
                br = 0.0;
                bg = 0.0;
                bb = 0.0;
                ba = a as f32 / 255.0;
            }

            // standard "over" compositing: top over outline
            let ta = top[3] as f32 / 255.0;
            let oa = ta + ba * (1.0 - ta);
            let (r, g, b) = if oa > 0.0 {
                (
                    (top[0] as f32 * ta + br * (1.0 - ta) * ba) / oa,
                    (top[1] as f32 * ta + bg * (1.0 - ta) * ba) / oa,
                    (top[2] as f32 * ta + bb * (1.0 - ta) * ba) / oa,
                )
            } else {
                (0.0, 0.0, 0.0)
            };

            out.put_pixel(
                x,
                y,
                Rgba([
                    r.round() as u8,
                    g.round() as u8,
                    b.round() as u8,
                    (oa * 255.0).round() as u8,
                ]),
            );
        }
    }

    out
}
