use crate::img_avg::img_avg::ImageAverager;

mod img_avg;

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 720;

fn main() 
{
    let mut avger = ImageAverager::new(
        "./src/source_images/",
        "./src/images_to_apply/",
        "./src/output_images/");

    avger.run(IMAGE_WIDTH, IMAGE_HEIGHT);
}
