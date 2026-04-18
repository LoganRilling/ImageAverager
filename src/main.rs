use image::{GrayImage, ImageBuffer, ImageReader};
use std::fs;
use log::error;

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 720;

fn main() 
{
    let source_paths = match fs::read_dir("./src/source_images/")
    {
        Ok(p) => p,
        Err(e) => {

            error!("Failed to load image directory: {e}");
            println!("Failed to load image directory: {e}");

            loop{}
        }
    };

    let source_folder_size: u32 = fs::read_dir("./src/source_images/").unwrap().count() as u32;

    let mut avg_img: GrayImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for path in source_paths
    {
        let file = match path
        {
            Ok(p) => p,
            Err(e) => {

                error!("Failed to load file: {e}");
                println!("Failed to load file: {e}");

                loop{}
            }
        };

        let p = file.path().display().to_string();
        println!("Path: {p}");

        let img = ImageReader::open(file.path().display().to_string()).unwrap()
            .with_guessed_format().unwrap().decode().unwrap();
        let img = match img.as_luma8()
        {
            Some(o) => o,
            None => {
                println!("File could not be used as luma8");
                loop {}
            }
        };

        let (width, height) = img.dimensions();

        for x in 0..width
        {
            for y in 0..height
            {           
                // avg_img[(x, y)][0] = (( (avg_img[(x, y)][0] as u32) + (img[(x, y)][0]) as u32) / source_folder_size) as u8; 
                // //Converts each pixel value you to u32 to avoid overflow, and then converts back to u8 after dividng by the folder size

                avg_img[(x, y)][0] = avg_img[(x, y)][0].saturating_add(img[(x, y)][0]);
            }
        }
    }

    for x in 0..IMAGE_WIDTH 
    {
        for y in 0..IMAGE_HEIGHT 
        {
            avg_img[(x, y)][0] = (avg_img[(x, y)][0] as u32 / source_folder_size) as u8;
        }
    }

    let _ = avg_img.save("./src/output_images/__average__.tiff");

    let apply_paths = match fs::read_dir("./src/images_to_apply/")
    {
        Ok(p) => p,
        Err(e) => {

            error!("Failed to load image directory: {e}");
            println!("Failed to load image directory: {e}");

            loop{}
        }
    };    

    for path in apply_paths
    {
        let file = match path
        {
            Ok(p) => p,
            Err(e) => {

                error!("Failed to load file: {e}");
                println!("Failed to load file: {e}");

                loop{}
            }
        };

        let main_img = ImageReader::open(file.path().display().to_string()).unwrap()
            .with_guessed_format().unwrap().decode().unwrap();
        let main_img = match main_img.as_luma8()
        {
            Some(o) => o,
            None => {
                println!("File could not be used as luma8");
                loop {}
            }
        };

        let mut output_img: GrayImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

        let (width, height) = main_img.dimensions();

        for x in 0..width
        {
            for y in 0..height
            {           
                let mut brightness_value = (main_img[(x, y)][0] as i16) - (avg_img[(x, y)][0] as i16);

                if brightness_value < 0
                {
                    brightness_value = 0;
                }

                output_img[(x, y)][0] = brightness_value as u8; 
                //Converts each pixel value you to u32 to avoid overflow, and then converts back to u8 after dividng by the folder size
            }
        }

        let output_name = String::from("./src/output_images/") + file.file_name().to_str().unwrap();

        let _ = output_img.save(output_name);        
    }
}
