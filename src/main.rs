use std::env;
use std::path::Path;
use image::{GenericImageView, DynamicImage, ImageBuffer, Luma};


static SMALL_WIDTH:     u32 = 32;    //32 char = 64 px
static MEDIUM_WIDTH:    u32 = 64;    //64 char = 128 px
static BIG_WIDTH:       u32 = 128;   //64 char = 256 px


fn main() {
    let mut size: String = String::from("medium");
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 { //path given ?
        if !Path::new(&args[1]).exists() { //path exist ? 
            panic!("{} not found", args[1]);
        }

        if args.len() <= 2{ //size given ?
            println!("no size given, taking medium by default");
        }else{
            if ["small", "medium", "big"].contains(&args[2].to_lowercase().as_str()) {
                size = args[2].to_lowercase();
                println!("image's new size given : \"{}\"", &size);
            }else {
                panic!("unkown size \"{}\"", &args[2]);
            }
        }

        let img: DynamicImage = image::open(&args[1]).unwrap();

        //RESIZING IMG
        let resized_img: DynamicImage = match size.as_str() {
            "small" => img.resize(SMALL_WIDTH*2, (img.height()*SMALL_WIDTH)/img.width(), image::imageops::FilterType::Gaussian),
            "medium" => img.resize(MEDIUM_WIDTH*2, (img.height()*MEDIUM_WIDTH)/img.width(), image::imageops::FilterType::Gaussian),
            "big" => img.resize(BIG_WIDTH*2, (img.height()*BIG_WIDTH)/img.width(), image::imageops::FilterType::Gaussian),
            _ => img.to_owned(),
        };
        println!("image resized from {:?} to {:?}", img.dimensions(), resized_img.dimensions());

        //RECOLOR IMG
        let mut recolored_img: ImageBuffer<Luma<u8>, Vec<u8>> = resized_img.grayscale().as_luma8().expect("can't turn img to black&white").to_owned();
        image::imageops::dither(&mut recolored_img, &image::imageops::BiLevel);
        println!("image successfully recolored to black&white");

        recolored_img.save("cat.png").unwrap();

    }else{
        panic!("Please provide a path to an image");
    }
}

