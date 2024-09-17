use anyhow::{anyhow, Result};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::collections::HashMap;

pub fn generate_word_cloud(words: HashMap<String, usize>, output: &str) -> Result<()> {
    let mut image = RgbImage::new(1000, 1000);
    let font = Font::try_from_bytes(include_bytes!("../assets/DejaVuSans.ttf"))
        .ok_or_else(|| anyhow!("Failed to load font file"))?;
    let mut y = 10.0;

    for (word, count) in words.iter().take(100) {
        let scale = Scale {
            x: *count as f32 * 2.0,
            y: *count as f32 * 2.0,
        };
        let color = Rgb([
            (*count * 7 % 200 + 55) as u8,
            (*count * 11 % 200 + 55) as u8,
            (*count * 17 % 200 + 55) as u8,
        ]);

        draw_text_mut(&mut image, color, 10, y as i32, scale, &font, word);
        y += scale.y + 5.0;
        if y > 990.0 {
            y = 10.0;
        }
    }

    image.save(output)?;
    Ok(())
}
