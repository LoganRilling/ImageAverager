pub mod img_avg
{
    use image::{GrayImage, ImageBuffer, Luma};

    // Stores the computed average grayscale image
    pub struct ImageAveragerFromBuffer
    {
        average_img: GrayImage,
    }

    #[allow(dead_code)]
    impl ImageAveragerFromBuffer
    {
        // Create an empty averager (0x0 image)
        pub fn new() -> ImageAveragerFromBuffer
        {
            return ImageAveragerFromBuffer 
            {
                average_img: ImageBuffer::new(0, 0)
            }
        }

        // Create averager and compute average from input images
        pub fn new_with_source(source: Vec<ImageBuffer<Luma<u8>, Vec<u8>>>) -> ImageAveragerFromBuffer
        {
            return ImageAveragerFromBuffer 
            {
                average_img: Self::find_average(source)
            }
        }

        // Compute per-pixel average across all images
        pub fn find_average(source: Vec<ImageBuffer<Luma<u8>, Vec<u8>>>) -> GrayImage
        {
            // Accumulator uses u32 to prevent overflow during summation
            let mut avg: ImageBuffer<Luma<u32>, Vec<u32>> =
                ImageBuffer::new(source[0].width(), source[0].height());

            // Final output image (u8 grayscale)
            let mut output: GrayImage =
                ImageBuffer::new(avg.width(), avg.height());

            // Number of input images
            let sample_size = source.iter().count() as u32;

            // Sum pixel values across all images
            for buf in source
            {
                for x in 0..buf.width()
                {
                    for y in 0..buf.height()
                    {
                        // Safe accumulation (no early clipping like u8)
                        avg[(x, y)][0] =
                            avg[(x, y)][0].saturating_add(buf[(x, y)][0] as u32); 
                    }
                }
            }

            // Normalize sum to get average
            for x in 0..avg.width()
            {
                for y in 0..avg.height()
                {
                    if sample_size > 0
                    {
                        // Divide once, then cast back to u8
                        output[(x, y)][0] =
                            (avg[(x, y)][0] / sample_size) as u8;
                    }
                    else 
                    {
                        // Fallback for empty input (should not normally occur)
                        output[(x, y)][0] = 0;    
                    }
                }
            }

            return output;
        }

        // Return a copy of the average image
        #[allow(dead_code)]
        pub fn get_average(&self) -> GrayImage
        {
            return self.average_img.clone();
        }

        // Subtract average image from input image (in-place)
        #[allow(dead_code)]
        pub fn apply_average(&self, img: ImageBuffer<Luma<u8>, Vec<u8>>) -> GrayImage
        {
            let mut output = img.clone();

            for x in 0..img.width()
            {
                for y in 0..img.height()
                {           
                    // Compute pixel difference using wider type to avoid underflow
                    let mut brightness_value =
                        (img[(x, y)][0] as i16) - (self.average_img[(x, y)][0] as i16);
                
                    // Clamp negatives to 0 (valid u8 range)
                    if brightness_value < 0
                    {
                        brightness_value = 0;
                    }
                
                    // Write adjusted pixel back
                    output[(x, y)][0] = brightness_value as u8; 
                }
            }

            return output;
        }
    }
}