//use std::{env::args, path::Path};

use image::{GenericImageView, ImageBuffer, RgbImage};

use std::{fmt::Debug, path::PathBuf};
use structopt::StructOpt;



fn get_bit(message: &str, index: usize) -> u8 {
    let bytes: Vec<char> = message.chars().collect();
    let byte: u8 = bytes[index/8] as u8;
    let bit_index = 7 - index % 8;
    let bit = (byte & (1 << bit_index)) >> bit_index;
    bit
}

fn write_message_to_image(image_path: &PathBuf, message: &str) -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let input_img = image::open(image_path).unwrap();
    let (width, height) = input_img.dimensions();
    let mut output_img: RgbImage = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let mut pixel = input_img.get_pixel(x, y);
            let index = (x + y * width as u32) as usize;
            if index < message.len() * 8 {
                let bit = get_bit(message, index as usize);
                pixel[0] = (pixel[0] & 0xfe) | bit;
                //pixel[1] = 255;
            } else {
                pixel[0] = (pixel[0] & 0xfe) | 1;
            }
            output_img.put_pixel(x, y, image::Rgb([pixel[0], pixel[1], pixel[2]]));

        }
    }
    output_img
}

fn read_message_from_image(image_path: &PathBuf) -> String {
    // Extracting message
    let mut message_image = image::open(image_path).unwrap().to_rgb8();
    let (width, height) = message_image.dimensions();
    let mut output_message_vec: Vec<u8> = vec![0; 1];
    for (x, y, pixel) in message_image.enumerate_pixels_mut() {
        let index = (x + y * width as u32) as usize;
        if index / 8 >= output_message_vec.len() {
            output_message_vec.push(0);
        }
        if index < (width * height) as usize  {
            let bit = pixel[0] & 0x1;
            // println!("{}", bit);
            output_message_vec[index/8] = output_message_vec[index/8] | (bit << (7- (index % 8)));
            // println!("{:?}", output_message[index/8]);
        }
        if index % 8 == 7 && output_message_vec[index/8] >= 128 {
            output_message_vec.pop();
            //println!("{:?}", output_message_vec[index/8]);
            break;
        }
    }
    let output_message: String = output_message_vec.iter().map(|&x| x as char).collect(); 
    
    output_message

}


#[derive(StructOpt, Debug)]
#[structopt(name = "image_steg")]
/// A commange line tool to hide messages inside image files.
enum Opt {
    /// Hide messages inside image files
    Encode{
        /// The source image to use
        #[structopt(parse(from_os_str))] 
        source: PathBuf,   
        /// Output image file
        #[structopt(parse(from_os_str))] 
        output: PathBuf, 
        /// The message string
        #[structopt(short="m", long="message")]
        message: String,       
    },
    /// Read messages from image files
    Decode {
        /// Image file containing the message
        #[structopt(parse(from_os_str))] 
        image: PathBuf,
    },
}


fn main() {
    let opt = Opt::from_args();
    //println!("{:?}", opt);
    match opt {
        Opt::Encode {source, output, message} => {
            println!("Encoding...");
            let message_image = write_message_to_image(&source, &message);
            message_image.save(&output).expect("Failed to create output image.");
            println!("Message saved to {:?}", output);
        },
        Opt::Decode {image} => {
            println!("Decoding from {:?}...", image);
            let message = read_message_from_image(&image);
            println!("Message: {}", message)
        }
    }
}