use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

type Color = [u8; 3];
type Pixels = usize;
type Pos = [Pixels; 2];

struct HashRandom {
    hasher: DefaultHasher,
}

impl HashRandom {
    fn new() -> Self {
        Self { hasher: DefaultHasher::new() }
    }

    fn next_u64(&mut self) -> u64 {
        self.hasher.write_u32(7);
        self.hasher.finish()
    }
}

struct Image<const W: Pixels, const H: Pixels> {
    pixels: Box<[Color]>,
}

struct Circle {
    radius: Pixels,
    pos: Pos,
}

impl<const W: Pixels, const H: Pixels> Image<W, H> {
    fn new() -> Self {
        Self {
            pixels: (vec![[0x00; 3]; W*H]).into_boxed_slice(),
        }
    }

    fn draw_circle(mut self, circle: Circle, color: Color) -> Self {
        type S = i64;

        let (cx, cy) = (circle.pos[0] as S, circle.pos[1] as S);
        let r = 2 * circle.radius as S;

        for x in 0..W {
            for y in 0..H {
                let (px, py) = (
                    2 * (x as S - cx) + 1,
                    2 * (y as S - cy) + 1,
                );
                if px * px + py * py <= r * r {
                    self.pixels[y * W + x] = color;
                }
            }
        }

        self
    }

    fn set_pixel(mut self, pos: Pos, color: Color) -> Self {
        self.pixels[pos[1] * W + pos[0]] = color;
        self
    }

    fn save_to_ppm(&self, file_name: &str) -> io::Result<()> {
        let mut file_buf = BufWriter::new(File::create(file_name)?);

        write!(&mut file_buf, "P6\n{} {} 255\n", W, H)?;

        for color in self.pixels.iter() {
            file_buf.write(color)?;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    const DIM: Pixels = 2 << 8;

    let mut image = Image::<DIM, DIM>::new();


    let mut rand = HashRandom::new();

    for _ in 0..1000 {
        let r = rand.next_u64() as Pixels % (DIM / 16);
        let w = rand.next_u64() as Pixels % DIM;
        let h = rand.next_u64() as Pixels % DIM;

        let red = (rand.next_u64() % 256) as u8;
        let green = (rand.next_u64() % 256) as u8;
        let blue = (rand.next_u64() % 256) as u8;

        image = image.draw_circle(
            Circle {
                radius: r,
                pos: [w, h],
            },
            [red, green, blue],
        );
    }

    image
        .set_pixel([2,2], [0xFF;3])
        .save_to_ppm("output/test_image.ppm")?;

    Ok(())
}
