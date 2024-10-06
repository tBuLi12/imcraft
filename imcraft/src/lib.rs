use std::path::Path;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub trait Image {
    fn get(&self, x: f32, y: f32) -> Pixel;

    fn transform(self, matrix: [[f32; 3]; 3]) -> impl Image + Sized
    where
        Self: Sized,
    {
        Transform {
            image: self,
            matrix: invert(matrix),
        }
    }

    fn join(self, other: impl Image) -> impl Image + Sized
    where
        Self: Sized,
    {
        Join {
            image1: self,
            image2: other,
        }
    }

    fn translate(self, x: f32, y: f32) -> impl Image + Sized
    where
        Self: Sized,
    {
        self.transform([[1.0, 0.0, x], [0.0, 1.0, y], [0.0, 0.0, 1.0]])
    }

    fn render(&self, width: usize, height: usize) -> Vec<u8> {
        let mut buf = vec![0; width * height * 4];
        for y in 0..height {
            for x in 0..width {
                let pixel = self.get(x as f32, y as f32);
                let idx = (y * width + x) * 4;
                buf[idx + 0] = (pixel.r * 255.0) as u8;
                buf[idx + 1] = (pixel.g * 255.0) as u8;
                buf[idx + 2] = (pixel.b * 255.0) as u8;
                buf[idx + 3] = (pixel.a * 255.0) as u8;
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

impl<I: Image> Image for &I {
    fn get(&self, x: f32, y: f32) -> Pixel {
        I::get(*self, x, y)
    }
}

// impl Image for Box<dyn Image> {
//     fn get(&self, x: f32, y: f32) -> Pixel {
//         let img: &dyn Image = &*self;
//         img.get(x, y)
//     }
// }

pub struct Uniform {
    color: Pixel,
}

impl Uniform {
    pub fn new(color: Pixel) -> Self {
        Self { color }
    }
}

impl Image for Uniform {
    fn get(&self, _x: f32, _y: f32) -> Pixel {
        self.color
    }
}

struct Transform<I> {
    image: I,
    matrix: [[f32; 3]; 3],
}

impl<I: Image> Transform<I> {
    fn transform(&self, x: f32, y: f32) -> (f32, f32) {
        let x2 = x * self.matrix[0][0] + y * self.matrix[0][1] + self.matrix[0][2];
        let y2 = x * self.matrix[1][0] + y * self.matrix[1][1] + self.matrix[1][2];
        (x2, y2)
    }
}

impl<I: Image> Image for Transform<I> {
    fn get(&self, x: f32, y: f32) -> Pixel {
        let (x2, y2) = self.transform(x, y);
        self.image.get(x2, y2)
    }
}

pub struct Join<I1, I2> {
    image1: I1,
    image2: I2,
}

impl<I1: Image, I2: Image> Image for Join<I1, I2> {
    fn get(&self, x: f32, y: f32) -> Pixel {
        let px1 = self.image1.get(x, y);
        let px2 = self.image2.get(x, y);
        let a = px2.a + px1.a * (1.0 - px2.a);
        if a == 0.0 {
            return Pixel {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            };
        }

        let blend = |v1, v2| (v2 * px2.a + v1 * px1.a * (1.0 - px2.a)) / a;
        Pixel {
            r: blend(px1.r, px2.r),
            g: blend(px1.g, px2.g),
            b: blend(px1.b, px2.b),
            a,
        }
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
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            };
        }

        let x = x.round() as usize;
        let y = y.round() as usize;
        if x >= self.width || y >= self.height {
            return Pixel {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            };
        }

        let idx = (y * self.width + x) * 4;
        let r = self.data[idx] as f32 / 255.0;
        let g = self.data[idx + 1] as f32 / 255.0;
        let b = self.data[idx + 2] as f32 / 255.0;
        let a = self.data[idx + 3] as f32 / 255.0;

        Pixel { r, g, b, a }
    }
}

fn invert(matrix: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    let mut adjoint = [
        [
            matrix[1][1] * matrix[2][2] - matrix[2][1] * matrix[1][2],
            matrix[0][2] * matrix[2][1] - matrix[0][1] * matrix[2][2],
            matrix[0][1] * matrix[1][2] - matrix[1][1] * matrix[0][2],
        ],
        [
            matrix[1][2] * matrix[2][0] - matrix[2][2] * matrix[1][0],
            matrix[0][0] * matrix[2][2] - matrix[0][2] * matrix[2][0],
            matrix[0][2] * matrix[1][0] - matrix[1][2] * matrix[0][0],
        ],
        [
            matrix[1][0] * matrix[2][1] - matrix[2][0] * matrix[1][1],
            matrix[0][1] * matrix[2][0] - matrix[0][0] * matrix[2][1],
            matrix[0][0] * matrix[1][1] - matrix[1][0] * matrix[0][1],
        ],
    ];
    let determinant =
        matrix[0][0] * adjoint[0][0] + matrix[0][1] * adjoint[1][0] + matrix[0][2] * adjoint[2][0];
    if determinant == 0.0 {
        return [[0.0; 3]; 3];
    }
    for i in 0..3 {
        for j in 0..3 {
            adjoint[i][j] /= determinant;
        }
    }
    adjoint
}
