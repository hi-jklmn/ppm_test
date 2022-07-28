use std::f32::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

type Color = [u8; 3];

trait IntoHSL {
    fn from_hsl(hsl_value: HSL) -> Self;
}

impl IntoHSL for Color {
    // https://www.wikiwand.com/en/HSL_and_HSV#/To_RGB
    fn from_hsl(hsl_value: HSL) -> Self {
        let hsl_value = hsl_value.rectified();
        let (h, s, l) = (hsl_value.h, hsl_value.s, hsl_value.l);

        let f = |n| {
            let k: f32 = (n + h / 30.0) % 12.0;
            let a: f32 = s * l.min(1.0 - l);

            l - a * (-1f32).max((k - 3f32).min(9f32 - k).min(1f32))
        };

        let (r, g, b) = (f(0.0), f(8.0), f(4.0));

        // TODO: round?
        [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
    }
}

struct HSL {
    h: f32,
    s: f32,
    l: f32,
}

impl HSL {
    fn rectified(&self) -> Self {
        Self {
            h: self.h % 360.0,
            s: self.s.clamp(0.0, 1.0),
            l: self.l.clamp(0.0, 1.0),
        }
    }
}

type Pixels = usize;
type Pos = [Pixels; 2];
type Dims = [Pixels; 2];

struct HashRandom {
    hasher: DefaultHasher,
}

macro_rules! next_type {
    ($fn_source:ident, $fn_name:ident, $type:ty) => {
        fn $fn_name(&mut self) -> $type {
            self.$fn_source() as $type
        }
    };
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

    next_type!(next_u64, next_u32, u32);
    next_type!(next_u64, next_u16, u16);
    next_type!(next_u64, next_u8, u8);

    fn next_f64(&mut self) -> f64 {
        self.hasher.write_u32(7);
        (self.hasher.finish() as f64) / (u64::MAX as f64)
    }

    next_type!(next_f64, next_f32, f32);
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

        let pos = self.pos;
        let radius = self.radius;

        let (min_x, max_x, min_y, max_y) = (
            pos[0].saturating_sub(radius),
            (pos[0] + radius).min(W),
            pos[1].saturating_sub(radius),
            (pos[1] + radius).min(H),
        );

        let (cx, cy) = (pos[0] as S, pos[1] as S);
        let r = 2 * radius as S;

        for x in min_x..max_x {
            for y in min_y..max_y {
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
    const DIM: Pixels = 2 << 10;

    let mut image = Image::<DIM, DIM>::new();

    let mut rand = HashRandom::seeded(4);

    let start = std::time::Instant::now();

    for r in (0..1 << 7).rev() {
        let w = rand.next_u64() as Pixels % DIM;
        let h = rand.next_u64() as Pixels % DIM;
        let theta = (rand.next_f32() * rand.next_f32() - 0.5) * 2.0 * PI;
        let (x, y) = {
            let radius = DIM as f32 / 3.0;
            let center = DIM as f32 / 2.0;
            (
                (radius * theta.cos() + center) as usize,
                (radius * theta.sin() + center) as usize,
            )
        };

        let red = rand.next_u8();
        let green = rand.next_u8();
        let blue = rand.next_u8();

        let radius = ((theta + PI) as f64) as Pixels * 45;

        let color = Color::from_hsl(HSL {
            h: -45.0 + rand.next_f32() * 90.0,
            s: rand.next_f32(),
            l: rand.next_f32(),
        });

        if rand.next_u8() & 1 > 0 {
            image = image.draw_shape(
                Circle {
                    pos: [x, y],
                    radius,
                },
                color,
            );
        } else {
            image = image.draw_shape(
                Rect {
                    pos: [x - radius / 2, y - radius / 2],
                    dim: [radius, radius],
                },
                color,
            );
        }
    }

    println!("Time: {:?}", start.elapsed());

    image.save_to_ppm("output/test_image.ppm")?;

    Ok(())
}
