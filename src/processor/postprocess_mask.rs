use image::imageops::FilterType;
use image::{Rgb, RgbImage};
use ndarray::{Array4, Axis};

/// Линейная интерполяция между двумя цветами (r,g,b)
#[inline]
fn lerp(a: (u8, u8, u8), b: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let (ar, ag, ab) = a;
    let (br, bg, bb) = b;
    let r = ar as f32 + (br as f32 - ar as f32) * t;
    let g = ag as f32 + (bg as f32 - ag as f32) * t;
    let b = ab as f32 + (bb as f32 - ab as f32) * t;
    (r.round() as u8, g.round() as u8, b.round() as u8)
}

/// Небольшая палитра-градиент: 0.0 = чёрный, 1.0 = белый
/// Точки: black → navy → blue → purple → red → orange → yellow → white
static STOPS: &[(f32, (u8, u8, u8))] = &[
    (0.00, (0, 0, 0)),       // black
    (0.15, (0, 0, 64)),      // navy
    (0.30, (0, 0, 255)),     // blue
    (0.45, (128, 0, 192)),   // purple
    (0.60, (255, 0, 0)),     // red
    (0.75, (255, 128, 0)),   // orange
    (0.90, (255, 255, 0)),   // yellow
    (1.00, (255, 255, 255)), // white
];

#[inline]
fn colormap(t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    for w in STOPS.windows(2) {
        let (t0, c0) = (w[0].0, w[0].1);
        let (t1, c1) = (w[1].0, w[1].1);
        if t <= t1 {
            let local = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
            return lerp(c0, c1, local);
        }
    }
    STOPS.last().unwrap().1
}

/// Визуализация маски как "тепловизора" без альфы.
/// gamma: >1.0 делает картинку «жёстче», <1.0 — мягче. Рекомендуем 0.8..1.2.
pub fn postprocess_mask(
    mask_output: &Array4<f32>,
    original_width: u32,
    original_height: u32,
) -> crate::Result<RgbImage> {
    let gamma: f32 = 1.2;

    // 1) Берём первый батч/канал как в исходной функции
    let temp_axis = mask_output.index_axis(Axis(0), 0);
    let mask_data = temp_axis.index_axis(Axis(0), 0);
    let (model_height, model_width) = mask_data.dim();

    // 2) Готовим LUT на 256 уровней (после гаммы)
    let g = gamma.clamp(0.2, 5.0);
    let mut lut = [(0u8, 0u8, 0u8); 256];
    for i in 0..256 {
        let t = (i as f32 / 255.0).powf(g);
        lut[i] = colormap(t);
    }

    // 3) Считаем "тепловизор" в RgbImage размера модели
    let mut heat = RgbImage::new(model_width as u32, model_height as u32);
    for (x, y, pixel) in heat.enumerate_pixels_mut() {
        // модель даёт logits → сигмоида
        let v = mask_data[[y as usize, x as usize]];
        let s = 1.0 / (1.0 + (-v).exp()); // 0..1
        let idx = (s * 255.0).round() as usize;
        let (r, g, b) = lut[idx.min(255)];
        *pixel = Rgb([r, g, b]);
    }

    // 4) Масштабируем в оригинальный размер. Для визуализации — Lanczos3 ок.
    let resized =
        image::imageops::resize(&heat, original_width, original_height, FilterType::Lanczos3);
    Ok(resized)
}
