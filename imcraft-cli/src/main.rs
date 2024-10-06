use imcraft::{Image, Pixel, Uniform};

fn main() {
    let tree = &imcraft::BufImage::open("tree.png");
    let squished = &tree.transform([[0.5, 0.0, 0.0], [0.0, 0.5, 0.0], [0.0, 0.0, 1.0]]);
    Uniform::new(Pixel {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    })
    .join(tree)
    .join(squished)
    .join(squished.translate(100.0, 0.0))
    .join(squished.translate(200.0, 0.0))
    .join(squished.translate(300.0, 0.0))
    .join(
        squished
            .transform([[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]])
            .translate(0.0, 512.0),
    )
    .write_to("tree2.png", 512, 512);
}
