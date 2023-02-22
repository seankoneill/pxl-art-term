use image::GenericImageView;
use ansi_colours::ansi256_from_rgb as ansi_from_rgb;
use ansi_term::Colour::Fixed;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];
    show_image(file_path);
}

fn show_image(path: &str) {
    let mut img = image::open(path).unwrap();

    let size: u32 = 32;
    img = img.resize(size,size,image::imageops::Nearest);

    let mut i: u32 = 0;
    for (_,_,px) in img.pixels() {
        let ansi: u8 = ansi_from_rgb::<[u8;3]>(
            px.0[0..3].try_into().unwrap()
        );

        print!("{}", Fixed(ansi).paint("\u{2588}"));
        i += 1;
        if i >= size {
            println!();
            i = 0;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn snorlax_example() {
        use crate::show_image;
        show_image("examples/snorlax.png");
    }

    #[test]
    fn pika_example() {
        use crate::show_image;
        show_image("examples/pika.png");
    }
}
