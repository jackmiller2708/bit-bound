use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let raw_dir = Path::new("assets/raw");
    let processed_dir = Path::new("assets/processed");

    if !processed_dir.exists() {
        fs::create_dir_all(processed_dir).expect("Failed to create assets/processed directory");
    }

    let entries = fs::read_dir(raw_dir).expect("Failed to read assets/raw");

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "png") {
            process_sprite(&path, processed_dir);
        }
    }
}

fn process_sprite(png_path: &Path, output_dir: &Path) {
    let file_stem = png_path.file_stem().unwrap().to_str().unwrap();
    let mut out_path = PathBuf::from(output_dir);
    out_path.push(format!("{}.2bpp", file_stem));

    println!("Processing: {:?} -> {:?}", png_path, out_path);

    let img = image::open(png_path)
        .expect("Failed to open image")
        .to_rgba8();

    let (width, height) = img.dimensions();
    let pixels = img.into_raw();

    // Map RGBA pixels to 2-bit palette indices
    let mut indexed: Vec<u8> = Vec::with_capacity((width * height) as usize);

    for px in pixels.chunks_exact(4) {
        let value = match px {
            [0, 0, 0, 0] => 0,        // transparent
            [15, 56, 15, 255] => 1,   // darkest green
            [48, 98, 48, 255] => 2,   // mid green
            [139, 172, 15, 255] => 3, // lightest green
            other => panic!("Unexpected color in sprite {:?}: {:?}", png_path, other),
        };

        indexed.push(value);
    }

    // Pad width/height to next multiple of 8
    let padded_width = (width as usize + 7) & !7;
    let padded_height = (height as usize + 7) & !7;

    let mut padded: Vec<u8> = vec![0; padded_width * padded_height];

    for y in 0..height as usize {
        for x in 0..width as usize {
            padded[y * padded_width + x] = indexed[y * width as usize + x];
        }
    }

    // Convert to 2bpp planar
    let tiles_x = padded_width / 8;
    let tiles_y = padded_height / 8;
    let mut output: Vec<u8> = Vec::with_capacity(tiles_x * tiles_y * 16);

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            for row in 0..8 {
                let py = ty * 8 + row;
                let mut low_byte: u8 = 0;
                let mut high_byte: u8 = 0;

                for col in 0..8 {
                    let px = tx * 8 + col;
                    let index = padded[py * padded_width + px];

                    let bit = 7 - col;
                    if index & 1 != 0 {
                        low_byte |= 1 << bit;
                    }
                    if index & 2 != 0 {
                        high_byte |= 1 << bit;
                    }
                }

                output.push(low_byte);
                output.push(high_byte);
            }
        }
    }

    let mut file = File::create(out_path).expect("Failed to create .2bpp file");
    file.write_all(&output).expect("Failed to write .2bpp file");
}
