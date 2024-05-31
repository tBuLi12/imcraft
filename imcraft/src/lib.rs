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
    fn size(&self) -> Size;
}

pub trait ImageValue: Sized + Image {
    fn transform(self, matrix: [[f32; 3]; 3]) -> impl ImageValue {
        Transform {
            image: self,
            matrix,
        }
    }

    fn write_to(&self, path: impl AsRef<Path>) {}
}

impl ImageValue for Box<dyn Image> {}
impl Image for Box<dyn Image> {
    fn get(&self, x: f32, y: f32) -> Pixel {
        let img: &dyn Image = &*self;
        img.get(x, y)
    }

    fn size(&self) -> Size {
        let img: &dyn Image = &*self;
        img.size()
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

impl<I: Image> ImageValue for Transform<I> {}
impl<I: Image> Image for Transform<I> {
    fn get(&self, x: f32, y: f32) -> Pixel {
        let (x2, y2) = self.transform(x, y);
        self.image.get(x2, y2)
    }

    fn size(&self) -> Size {
        let size = self.image.size();
        match size {}

        Size { width, height }
    }
}

struct BufImage {
    data: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Image for BufImage {
    fn get(&self, x: f32, y: f32) -> Pixel {
        self.data[y * self.width + x]
    }

    fn size(&self) -> Size {
        Size {
            width: Some(self.width),
            height: Some(self.height),
        }
    }
}
