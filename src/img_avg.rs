pub mod img_avg
{
    use image::{GrayImage, ImageBuffer, ImageReader};
    use std::fs;
    use log::error;

    // Main struct holding directory paths and state
    pub struct ImageAverager
    {
        pub image_source_path: String, // Directory of source images for averaging
        pub image_apply_path: String,  // Directory of images to apply average to
        pub image_output_path: String, // Output directory
        fail_state: bool               // Indicates invalid state
    }

    impl ImageAverager
    {
        // Hard stop on unrecoverable error
        fn handle_run_error() -> !
        {
            // No concrete error handler as of right now. Currently just loops to stop the process
            loop{}
        }

        // Constructor using &str paths
        #[allow(dead_code)]
        pub fn new(image_source_path: &str, image_apply_path: &str, image_output_path: &str) -> ImageAverager
        {
            let mut fail_state = false;
            let mut source_path = "".to_string();
            let mut apply_path = "".to_string();
            let mut output_path = "".to_string();

            // Validate source directory
            match fs::read_dir(image_source_path)
            {
                Ok(_p) => 
                {
                    source_path = image_source_path.to_string();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load source image directory: {e}");
                    println!("ERROR! Failed to load source image directory: {e}");

                    fail_state = true;
                }
            };

            // Validate apply directory
            match fs::read_dir(image_apply_path)
            {
                Ok(_p) =>
                {
                    apply_path = image_apply_path.to_string();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load apply image directory: {e}");
                    println!("ERROR! Failed to load apply image directory: {e}");

                    fail_state = true;
                }
            };

            // Validate output directory
            match fs::read_dir(image_output_path)
            {
                Ok(_p) =>
                {
                    output_path = image_output_path.to_string();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load output image directory: {e}");
                    println!("ERROR! Failed to load output image directory: {e}");

                    fail_state = true;
                }
            };


            return ImageAverager 
            {
                image_source_path: source_path,
                image_apply_path: apply_path,
                image_output_path: output_path,
                fail_state
            }
        }

        // Constructor using String references
        #[allow(dead_code)]
        pub fn new_with_string(image_source_path: &String, image_apply_path: &String, image_output_path: &String) -> ImageAverager
        {
            let mut fail_state = false;
            let mut source_path = "".to_string();
            let mut apply_path = "".to_string();
            let mut output_path = "".to_string();

            // Validate source directory
            match fs::read_dir(image_source_path)
            {
                Ok(_p) => 
                {
                    source_path = image_source_path.clone();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load source image directory: {e}");
                    println!("ERROR! Failed to load source image directory: {e}");

                    fail_state = true;
                }
            };

            // Validate apply directory
            match fs::read_dir(image_apply_path)
            {
                Ok(_p) =>
                {
                    apply_path = image_apply_path.clone();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load apply image directory: {e}");
                    println!("ERROR! Failed to load apply image directory: {e}");

                    fail_state = true;
                }
            };

            // Validate output directory
            match fs::read_dir(image_output_path)
            {
                Ok(_p) =>
                {
                    output_path = image_output_path.clone();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load output image directory: {e}");
                    println!("ERROR! Failed to load output image directory: {e}");

                    fail_state = true;
                }
            };


            return ImageAverager 
            {
                image_source_path: source_path,
                image_apply_path: apply_path,
                image_output_path: output_path,
                fail_state
            }
        }

        // Set source directory with validation
        #[allow(dead_code)]
        pub fn set_image_source_path(&mut self, path: &String)
        {
            match fs::read_dir(path)
            {
                Ok(_p) => 
                {
                    self.image_source_path = path.clone();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load source image directory: {e}");
                    println!("ERROR! Failed to load source image directory: {e}");

                    self.fail_state = true;
                }
            };
        }

        // Set apply directory with validation
        #[allow(dead_code)]
        pub fn set_image_apply_path(&mut self, path: &String)
        {
            match fs::read_dir(path)
            {
                Ok(_p) =>
                {
                    self.image_apply_path = path.clone();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load apply image directory: {e}");
                    println!("ERROR! Failed to load apply image directory: {e}");

                    self.fail_state = true;
                }
            };
        }

        // Set output directory with validation
        #[allow(dead_code)]
        pub fn set_image_output_path(&mut self, path: &String)
        {
            match fs::read_dir(path)
            {
                Ok(_p) =>
                {
                    self.image_output_path = path.clone();
                },
                Err(e) => 
                {
                    error!("ERROR! Failed to load output image directory: {e}");
                    println!("ERROR! Failed to load output image directory: {e}");

                    self.fail_state = true;
                }
            };
        }

        // Re-check all paths; returns overall validity
        #[allow(dead_code)]
        pub fn refresh_status(&mut self) -> bool
        {

            // Check source directory
            match fs::read_dir(self.image_source_path.clone())
            {
                Ok(p) => p,
                Err(e) => 
                {
                    error!("ERROR! Failed to load source image directory: {e}");
                    println!("ERROR! Failed to load source image directory: {e}");

                    self.fail_state = false;
                    return false;
                }
            };

            // Check apply directory
            match fs::read_dir(self.image_apply_path.clone())
            {
                Ok(p) => p,
                Err(e) => 
                {
                    error!("ERROR! Failed to load apply image directory: {e}");
                    println!("ERROR! Failed to load apply image directory: {e}");

                    self.fail_state = false;
                    return false;
                }
            };

            // Check output directory
            match fs::read_dir(self.image_output_path.clone())
            {
                Ok(p) => p,
                Err(e) => 
                {
                    error!("ERROR! Failed to load output image directory: {e}");
                    println!("ERROR! Failed to load output image directory: {e}");

                    self.fail_state = false;
                    return false;
                }
            };

            self.fail_state = true;
            return true;
        }

        // Core execution: builds average image and applies subtraction
        #[allow(dead_code)]
        pub fn run(&mut self, image_width: u32, image_height: u32)
        {
            // Abort if object is invalid
            if self.fail_state
            {
                error!("ERROR! Image Averager Object is in the fail state! Can not run!");
                println!("ERROR! Image Averager Object is in the fail state! Can not run!");
            }
            else
            {
                let source_dir = fs::read_dir(self.image_source_path.clone()).unwrap();
                let source_size: u32 = fs::read_dir(self.image_source_path.clone()).unwrap().count() as u32;

                // Accumulator image for averaging
                let mut avg_img: GrayImage = ImageBuffer::new(image_width, image_height);

                // Sum all source images
                for path in source_dir
                {
                    let file = match path
                    {
                        Ok(p) => p,
                        Err(e) => 
                        {
                            error!("ERROR! Failed to load file: {e}");
                            println!("ERROR! Failed to load file: {e}");
                        
                            Self::handle_run_error();
                        }
                    };
                
                    // Load and decode image
                    let img = ImageReader::open(file.path().display().to_string()).unwrap()
                        .with_guessed_format().unwrap().decode().unwrap();
                    
                    // Convert to grayscale (Luma8)
                    let img = match img.as_luma8()
                    {
                        Some(o) => o,
                        None => 
                        {
                            error!("ERROR! File could not be used as Luma8!");
                            println!("ERROR! File could not be used as Luma8!");
                        
                            Self::handle_run_error();
                        }
                    };
                
                    let (width, height) = img.dimensions();
                
                    // Accumulate pixel brightness
                    for x in 0..width
                    {
                        for y in 0..height
                        {
                            avg_img[(x, y)][0] = avg_img[(x, y)][0].saturating_add(img[(x, y)][0]); 
                        }
                    }
                }

                // Divide by number of images to compute average
                for x in 0..image_width
                {
                    for y in 0..image_height 
                    {
                        avg_img[(x, y)][0] = (avg_img[(x, y)][0] as u32 / source_size) as u8;
                    }
                }

                // Save average image
                let _ = avg_img.save(self.image_output_path.clone() + "/__average__.tiff");


                let apply_paths = fs::read_dir(self.image_apply_path.clone()).unwrap();

                // Process apply images
                for path in apply_paths
                {
                    let file = match path
                    {
                        Ok(p) => p,
                        Err(e) => 
                        {
                            error!("ERROR! Failed to load file: {e}");
                            println!("ERROR! Failed to load file: {e}");
                        
                            Self::handle_run_error();
                        }
                    };
                
                    // Load and convert image
                    let img = ImageReader::open(file.path().display().to_string()).unwrap()
                        .with_guessed_format().unwrap().decode().unwrap();
                    
                    let img = match img.as_luma8()
                    {
                        Some(o) => o,
                        None => 
                        {
                            error!("ERROR! File could not be used as Luma8!");
                            println!("ERROR! File could not be used as Luma8!");
                        
                            Self::handle_run_error();
                        }
                    };
                
                
                    // Output image buffer
                    let mut output_img: GrayImage = ImageBuffer::new(image_width, image_height);
                
                
                    let (width, height) = img.dimensions();
                
                
                    // Subtract average from each pixel
                    for x in 0..width
                    {
                        for y in 0..height
                        {           
                            let mut brightness_value = (img[(x, y)][0] as i16) - (avg_img[(x, y)][0] as i16);
                        
                            // Clamp to zero
                            if brightness_value < 0
                            {
                                brightness_value = 0;
                            }
                        
                            output_img[(x, y)][0] = brightness_value as u8; 
                            //Converts each pixel value you to u32 to avoid overflow, and then converts back to u8 after div
                        }
                    }
                
                    // Build output file path
                    let output_name = self.image_output_path.clone() + "/" + file.file_name().to_str().unwrap();
                
                    // Save processed image
                    let _ = output_img.save(output_name);        
                }
            }
        }
    }
}