use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let frames = [
        ("assets/spaceship_0.png", "PLAYER_SPRITE_FRAME_1"),
        ("assets/spaceship_1.png", "PLAYER_SPRITE_FRAME_2"),
    ];

    let mut output = String::new();

    for (path, name) in frames {
        let img = image::open(path).expect("Failed to open image").to_rgba8();

        let (width, height) = img.dimensions();
        assert!(width == 35 && height == 16, "Sprite must be 35x16");

        let pixels = img.into_raw();

        let mut indexed: Vec<u8> = Vec::with_capacity((width * height) as usize);

        for px in pixels.chunks_exact(4) {
            let value = match px {
                [0, 0, 0, 0] => 0, // transparent

                // darkest green
                [15, 56, 15, 255] => 1,

                // mid green
                [48, 98, 48, 255] => 2,

                // lightest green
                [139, 172, 15, 255] => 3,

                other => {
                    panic!("Unexpected color in sprite: {:?}", other);
                }
            };

            indexed.push(value);
        }

        assert_eq!(indexed.len(), 560);

        output.push_str(&format!("pub const {}: [u8; 560] = [\n", name));

        for (i, p) in indexed.iter().enumerate() {
            output.push_str(&format!("{},", p));

            if (i + 1) % width as usize == 0 {
                output.push('\n');
            }
        }

        output.push_str("];\n\n");
    }

    let out_path = Path::new("src/sprites.rs");
    let mut file = File::create(out_path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
