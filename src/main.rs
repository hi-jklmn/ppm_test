use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

type Color = [u8; 3];

type Pixels = usize;
type Pos = [Pixels; 2];

struct Image<const W: Pixels, const H: Pixels> {
    pixels: Box<[Color]>,
}

#[allow(unused)]
struct Rect<const W: Pixels, const H: Pixels> {
    pos: Pos,
}

#[allow(unused)]
struct Ellipse<const W: Pixels, const H: Pixels> {
    pos: Pos,
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
        let r = circle.radius as S;

        for x in 0..W {
            for y in 0..H {
                let (px, py) = (x as S - cx, y as S - cy);
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

    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();

    for i in 0..10 {
        hasher.write_u32(7 * (i + 54));
        let r = hasher.finish() as Pixels % DIM;
        hasher.write_u32(7 * (i + 54));
        let w = hasher.finish() as Pixels % DIM;
        hasher.write_u32(7 * (i + 54));
        let h = hasher.finish() as Pixels % DIM;

        hasher.write_u32(7 * (i + 54));
        let red = (hasher.finish() % 256) as u8;
        hasher.write_u32(7 * (i + 54));
        let green = (hasher.finish() % 256) as u8;
        hasher.write_u32(7 * (i + 54));
        let blue = (hasher.finish() % 256) as u8;

        image = image.draw_circle(
            Circle {
                radius: r / 4,
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
