use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

type Color = [u8; 3];
type Pixels = usize;
type Pos = [Pixels; 2];
type Dims = [Pixels; 2];

struct HashRandom {
    hasher: DefaultHasher,
}

impl HashRandom {
    fn new() -> Self {
        Self {
            hasher: DefaultHasher::new(),
        }
    }

    fn seeded(seed: u32) -> Self {
        let mut ret = Self::new();
        ret.seed(seed);
        ret
    }

    fn seed(&mut self, seed: u32) {
        self.hasher = DefaultHasher::new();
        self.hasher.write_u32(seed);
    }

    fn next_u64(&mut self) -> u64 {
        self.hasher.write_u32(7);
        self.hasher.finish()
    }
}

struct Image<const W: Pixels, const H: Pixels> {
    pixels: Box<[Color]>,
}

impl<const W: Pixels, const H: Pixels> Image<W, H> {
    fn width() -> Pixels {
        W
    }
    fn height() -> Pixels {
        H
    }
}

trait Shape {
    fn draw<const W: Pixels, const H: Pixels>(&self, img: &mut Image<W, H>, color: Color);
}

struct Circle {
    pos: Pos,
    radius: Pixels,
}

impl Shape for Circle {
    fn draw<const W: Pixels, const H: Pixels>(&self, image: &mut Image<W, H>, color: Color) {
        type S = i64;

        let (cx, cy) = (self.pos[0] as S, self.pos[1] as S);
        let r = 2 * self.radius as S;

        for x in 0..W {
            for y in 0..H {
                let (px, py) = (2 * (x as S - cx) + 1, 2 * (y as S - cy) + 1);
                if px * px + py * py <= r * r {
                    image.pixels[y * W + x] = color;
                }
            }
        }
    }
}

struct Rect {
    pos: Pos,
    dim: Dims,
}

impl Shape for Rect {
    fn draw<const W: Pixels, const H: Pixels>(&self, image: &mut Image<W, H>, color: Color) {
        let (min_x, max_x, min_y, max_y) = (
            self.pos[0].min(W),
            (self.pos[0] + self.dim[0] + 1).min(W),
            self.pos[1].min(H),
            (self.pos[1] + self.dim[1] + 1).min(H),
        );

        for x in min_x..max_x {
            for y in min_y..max_y {
                image.pixels[y * W + x] = color;
            }
        }
    }
}

struct Pixel {
    pos: Pos,
}

impl Shape for Pixel {
    fn draw<const W: Pixels, const H: Pixels>(&self, image: &mut Image<W, H>, color: Color) {
        image.pixels[self.pos[1] * W + self.pos[0]] = color;
    }
}

impl<const W: Pixels, const H: Pixels> Image<W, H> {
    fn new() -> Self {
        Self {
            pixels: (vec![[0x00; 3]; W * H]).into_boxed_slice(),
        }
    }

    fn draw_shape(mut self, shape: impl Shape, color: Color) -> Self {
        shape.draw(&mut self, color);
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

    let mut rand = HashRandom::seeded(2);

    for _ in 0..1000 {
        let w = rand.next_u64() as Pixels % DIM;
        let h = rand.next_u64() as Pixels % DIM;
        let x = rand.next_u64() as Pixels % DIM;
        let y = rand.next_u64() as Pixels % DIM;

        let red = (rand.next_u64() % 256) as u8;
        let green = (rand.next_u64() % 256) as u8;
        let blue = (rand.next_u64() % 256) as u8;

        image = image.draw_shape(
            Rect {
                pos: [x, y],
                dim: [w, h],
            },
            [red, green, blue],
        );
    }

    const N_TESTS: usize = 10000;
    const N_BINS: usize = 128;
    let mut bins = [0; N_BINS];

    let range_size = u64::MAX / N_BINS as u64;

    for _ in 0..N_TESTS {
        //for (i, bit) in format!("{:064b}", rand.next_u64()).chars().enumerate() {
        //    if bit == '1' {
        //        bins[i] += 1;
        //    }
        //}
        bins[(rand.next_u64() / range_size) as usize] += 1;
    }

    let spacing = DIM / N_BINS;

    for i in 0..N_BINS {
        let height = (bins[i] * DIM * 64) / N_TESTS;
        image = image.draw_shape(
            Rect {
                pos: [i * spacing, DIM - height],
                dim: [spacing - 2, height],
            },
            [0xFF; 3],
        );
    }

    image.save_to_ppm("output/test_image.ppm")?;

    Ok(())
}
