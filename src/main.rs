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

#[allow(unused)]
impl HashRandom {
    fn new() -> Self {
        Self { hasher: DefaultHasher::new() }
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

#[allow(unused)]
impl<const W: Pixels, const H: Pixels> Image<W,H> {
    fn width() -> Pixels { W }
    fn height() -> Pixels { H }
}

struct Circle {
    pos: Pos,
    radius: Pixels,
}

struct Rect {
    pos: Pos,
    dim: Dims,
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

    fn draw_rect(mut self, rect: Rect, color: Color) -> Self {
        let (min_x, max_x, min_y, max_y) = 
            (rect.pos[0].min(W),
            (rect.pos[0] + rect.dim[0] + 1).min(W),
            rect.pos[1].min(H),
            (rect.pos[1] + rect.dim[1] + 1).min(H),
        );

        for x in min_x..max_x {
            for y in min_y..max_y {
                self.pixels[y * W + x] = color;
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


    let mut rand = HashRandom::seeded(2);

    for _ in 0..1000 {
        let w = rand.next_u64() as Pixels % DIM;
        let h = rand.next_u64() as Pixels % DIM;
        let x = rand.next_u64() as Pixels % DIM;
        let y = rand.next_u64() as Pixels % DIM;

        let red = (rand.next_u64() % 256) as u8;
        let green = (rand.next_u64() % 256) as u8;
        let blue = (rand.next_u64() % 256) as u8;

        image = image.draw_rect(
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
        image = image.draw_rect(
            Rect {
                pos: [i * spacing, DIM - height],
                dim: [spacing - 2, height]
            },
            [0xFF; 3],
        );
    }

    image.save_to_ppm("output/test_image.ppm")?;

    Ok(())
}
