use std::fs::File;
use std::io::{Write, BufWriter};
use std::io;

type Color = [u8; 3];

type Pixels = usize;
type Dims = [Pixels; 2];
type Pos = [Pixels; 2];

struct Image<const W: Pixels, const H: Pixels>{
    dims: Dims,
    pixels: Box<[[Color; W]]>,
}

#[derive(Default)]
struct Point {
    point: Dims,
}

struct Rect {
    dims: Dims,
    pos: Pos,
}

#[allow(unused)]
struct Ellipse {
    dims: Dims,
    pos: Pos,
}

#[derive(Default)]
struct Circle {
    radius: Pixels,
    pos: Pos,
}

impl Circle {
    fn new() -> Self {
        Circle::default()
    }
}

impl<const W: Pixels, const H: Pixels> Image<W, H> {
    fn new() -> Self {
        Self { dims: [W, H], pixels: (vec![[[0x00; 3]; W]; H]).into_boxed_slice()}
    }

    fn draw_circle(mut self, circle: Circle, color: Color) -> Self {
        type S = i64;

        let (cx, cy) = (circle.pos[0] as S, circle.pos[1] as S);
        let r = circle.radius as S;

        for x in 0..W {
            for y in 0..H {
                let (px, py) =  (x as S - cx, y as S - cy);
                if px*px + py*py <= r*r {
                    self.pixels[x][y] = color;
                }
            }
        }
                

        self
    }

    fn save_to_ppm(&self, file_name: &str) -> io::Result<()> {
        let mut file_buf = BufWriter::new(File::create(file_name)?);

        write!(&mut file_buf, "P6\n{} {} 255\n", W, H)?;

        for x in self.pixels.iter().flatten() {
            file_buf.write(x)?;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    const DIM : Pixels = 2<<8;

    let mut image = Image::<DIM,DIM>::new();
        //.draw_circle(
        //    Circle { radius: DIM / 2, pos: [DIM / 2, DIM / 2] },
        //    [0x00,0xFF,0xFF]
        //).save_to_ppm("test_image.ppm")?;
        //
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
            Circle { radius: r/4, pos: [w, h] },
            [red,green,blue]
        );
    }

    image.save_to_ppm("output/test_image.ppm")?;

    Ok(())
}
