use crate::error::RembgError;
use image::DynamicImage;
use oxipng::{Options, StripChunks, optimize_from_memory, Interlacing, Deflaters};

pub fn compress_png(image: &DynamicImage) -> Result<Vec<u8>, RembgError> {
    // 1) RGBA8
    let rgba = image.to_rgba8();
    let (w, h) = rgba.dimensions();

    // 2) Подготовить Vec<imagequant::RGBA>
    let raw = rgba.as_raw(); // &[u8] RGBA
    let mut pixels = Vec::with_capacity((w * h) as usize);
    for px in raw.chunks_exact(4) {
        pixels.push(imagequant::RGBA {
            r: px[0],
            g: px[1],
            b: px[2],
            a: px[3],
        });
    }

    // 3) Квантование (TinyPNG-стайл)
    let mut attr = imagequant::Attributes::new();
    attr.set_quality(60, 100)?; // перцептуальное качество (0..100)
    attr.set_speed(3)?; // 1 — лучше/медленнее, 10 — быстрее/хуже
    let mut img = attr.new_image(pixels, w as usize, h as usize, 0.0)?; // 0.0 = sRGB :contentReference[oaicite:0]{index=0}
    let mut qres = attr.quantize(&mut img)?; // генерим палитру :contentReference[oaicite:1]{index=1}

    // 4) Получить палитру и индексный буфер
    let (palette, indexed) = qres.remapped(&mut img)?; // (Vec<RGBA>, Vec<u8>) :contentReference[oaicite:2]{index=2}

    // 5) Собрать палеточный PNG (PLTE + tRNS)
    let mut pal_png = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut pal_png, w, h);
        enc.set_color(png::ColorType::Indexed);
        enc.set_depth(png::BitDepth::Eight);

        let mut plte = Vec::with_capacity(3 * palette.len());
        let mut trns = Vec::with_capacity(palette.len());
        for p in &palette {
            plte.extend_from_slice(&[p.r, p.g, p.b]);
            trns.push(p.a);
        }
        enc.set_palette(plte);
        enc.set_trns(trns);

        let mut writer = enc.write_header()?;
        writer.write_image_data(&indexed)?; // ждёт &[u8] индексов (1 байт/пиксель) :contentReference[oaicite:3]{index=3}
        writer.finish()?;
    }

    // 6) Lossless-оптимизация контейнера PNG (oxipng + zopfli)
    let mut opt = Options::from_preset(4);
    opt.strip = StripChunks::Safe;
    opt.optimize_alpha = true;
    opt.interlace = Some(Interlacing::None);
    opt.deflate = Deflaters::Libdeflater { compression: 12 };
    let optimized = optimize_from_memory(&pal_png, &opt)?;

    Ok(optimized)
}
