use std::path::Path;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Size {
    pub width: Option<usize>,
    pub height: Option<usize>,
}

pub trait Image {
    fn get(&self, x: f32, y: f32) -> Pixel;

    fn transform(self, matrix: [[f32; 3]; 3]) -> impl Image + Sized
    where
        Self: Sized,
    {
        Transform {
            image: self,
            matrix,
        }
    }

    fn render(&self, width: usize, height: usize) -> Vec<u8> {
        let mut buf = vec![0; width * height * 4];
        for y in 0..height {
            for x in 0..width {
                let pixel = self.get(x as f32, y as f32);
                let idx = (y * width + x) * 4;
                buf[idx] = pixel.r;
                buf[idx + 1] = pixel.g;
                buf[idx + 2] = pixel.b;
                buf[idx + 3] = pixel.a;
            }
        }
        buf
    }

    fn write_to(&self, path: impl AsRef<Path>, width: usize, height: usize) {
        let buf = self.render(width, height);
        image::save_buffer(
            path,
            &buf,
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        )
        .unwrap();
    }
}

// impl Image for Box<dyn Image> {
//     fn get(&self, x: f32, y: f32) -> Pixel {
//         let img: &dyn Image = &*self;
//         img.get(x, y)
//     }
// }

struct Transform<I> {
    image: I,
    matrix: [[f32; 3]; 3],
}

impl<I: Image> Transform<I> {
    fn transform(&self, x2: f32, y2: f32) -> (f32, f32) {
        // let x2 = x * self.matrix[0][0] + y * self.matrix[0][1] + self.matrix[0][2];
        // let y2 = x * self.matrix[1][0] + y * self.matrix[1][1] + self.matrix[1][2];

        let y = (x2
            - self.matrix[0][2]
            - (y2 - self.matrix[1][2]) / self.matrix[1][0] * self.matrix[0][0])
            / (self.matrix[0][1] - self.matrix[1][1] / self.matrix[1][0] * self.matrix[0][0]);

        // let y = (x2 - self.matrix[0][2])
        //     / (self.matrix[0][1] - self.matrix[1][1] * self.matrix[0][0] / self.matrix[1][0])
        //     - (y2 - self.matrix[1][2]) * self.matrix[0][0] / self.matrix[1][0];
        let x = (y2 - self.matrix[1][2] - y * self.matrix[1][1]) / self.matrix[1][0];

        (x, y)
    }
}

impl<I: Image> Image for Transform<I> {
    fn get(&self, x: f32, y: f32) -> Pixel {
        let (x2, y2) = self.transform(x, y);
        self.image.get(x2, y2)
    }
}

pub struct BufImage {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl BufImage {
    pub fn open(path: impl AsRef<Path>) -> Self {
        let data = image::ImageReader::open(path)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();
        BufImage {
            width: data.width() as usize,
            height: data.height() as usize,
            data: data.into_raw(),
        }
    }
}

impl Image for BufImage {
    fn get(&self, x: f32, y: f32) -> Pixel {
        if x < 0.0 || y < 0.0 {
            return Pixel {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            };
        }

        let x = x.round() as usize;
        let y = y.round() as usize;
        if x >= self.width || y >= self.height {
            return Pixel {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            };
        }

        let idx = (y * self.width + x) * 4;
        let r = self.data[idx];
        let g = self.data[idx + 1];
        let b = self.data[idx + 2];
        let a = self.data[idx + 3];

        Pixel { r, g, b, a }
    }
}
