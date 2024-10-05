use imcraft::Image;

fn main() {
    println!("{}", 1.0 / 2.0 * 2.0);
    imcraft::BufImage::open("tree.png")
        .transform([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        .write_to("tree2.png", 256, 256);
}
