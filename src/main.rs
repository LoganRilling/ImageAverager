use std::fs;
use image::{ImageBuffer, ImageReader, Luma};

use crate::img_avg_buf::img_avg::ImageAveragerFromBuffer;

mod img_avg_buf;
mod img_avg_files;

fn main() 
{

    

    let source_dir = fs::read_dir("./src/source_images/").unwrap();
    let apply_dir = fs::read_dir("./src/images_to_apply/").unwrap();

    let mut source_buf_vec: Vec<ImageBuffer<Luma<u8>, Vec<u8>>> = vec![];
    let mut apply_buf_vec: Vec<ImageBuffer<Luma<u8>, Vec<u8>>> = vec![];

    for path in source_dir
    {
        let img = ImageReader::open(path.unwrap().path().display().to_string()).unwrap();
        let img = img.with_guessed_format().unwrap();
        let img = img.decode().unwrap();
        let img = img.as_luma8().unwrap();

        source_buf_vec.push(img.clone());
    }

    for path in apply_dir
    {
        let img = ImageReader::open(path.unwrap().path().display().to_string()).unwrap();
        let img = img.with_guessed_format().unwrap();
        let img = img.decode().unwrap();
        let img = img.as_luma8().unwrap();

        apply_buf_vec.push(img.clone());
    }

    let avger = ImageAveragerFromBuffer::new_with_source(source_buf_vec);

    let mut i = 0;
    for buf in apply_buf_vec
    {
        let output_path = String::from("./src/output_images/") + i.to_string().as_str() + ".tiff";

        let _ = avger.apply_average(buf).save(output_path);

        i = i + 1;
    }

}
