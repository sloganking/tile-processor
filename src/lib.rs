pub mod tiler {
    use glob::{glob, GlobError};
    use image::{
        imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage,
    };
    use std::{collections::HashMap, fs, path::PathBuf};

    const IMAGE_TILE_WIDTH: i32 = 256;
    const IMAGE_TILE_HEIGHT: i32 = 256;

    #[derive(Debug)]
    struct Bounds {
        max_x: i32,
        max_z: i32,
        min_x: i32,
        min_z: i32,
    }

    struct FilenameAndNumbers {
        file_name: PathBuf,
        x: i32,
        z: i32,
    }

    /// Returns a list of all files in a directory and it's subdirectories
    pub fn get_files_in_dir(path: &str, filetype: &str) -> Result<Vec<PathBuf>, GlobError> {
        //> get list of all files and dirs in path, using glob
            let mut paths = Vec::new();

            let mut potential_slash = "";
            if PathBuf::from(path).is_dir() && !path.ends_with('/') {
                potential_slash = "/";
            }

            let search_params = String::from(path) + potential_slash + "**/*" + filetype;

            for entry in glob(&search_params).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        paths.push(path);
                    }
                    Err(e) => return Err(e),
                }
            }

        //<> filter out directories
            let paths = paths.into_iter().filter(|e| e.is_file());
        //<

        let paths: Vec<PathBuf> = paths.into_iter().collect();
        Ok(paths)
    }

    pub fn consolidate_images(files: &[PathBuf]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut bounds = Bounds {
            max_x: i32::MIN,
            max_z: i32::MIN,
            min_x: i32::MAX,
            min_z: i32::MAX,
        };

        let mut filename_and_numbers_vec: Vec<FilenameAndNumbers> = Vec::new();

        // find max and min dimensions
        for file in files {
            let file_name = file.file_stem().unwrap().to_str().unwrap();
            let split: Vec<&str> = file_name.split(',').collect();

            let x: i32 = split[0].parse().unwrap();
            let z: i32 = split[1].parse().unwrap();

            filename_and_numbers_vec.push(FilenameAndNumbers {
                file_name: file.clone(),
                x,
                z,
            });

            if x > bounds.max_x {
                bounds.max_x = x;
            }
            if z > bounds.max_z {
                bounds.max_z = z;
            }
            if x < bounds.min_x {
                bounds.min_x = x;
            }
            if z < bounds.min_z {
                bounds.min_z = z;
            }
        }

        let xdiff = bounds.max_x - bounds.min_x + 1;
        let zdiff = bounds.max_z - bounds.min_z + 1;

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut output_imgbuf = RgbaImage::new(
            (xdiff * IMAGE_TILE_WIDTH).try_into().unwrap(),
            (zdiff * IMAGE_TILE_HEIGHT).try_into().unwrap(),
        );

        for file_struc in &filename_and_numbers_vec {
            let tile_img = image::open(&file_struc.file_name).unwrap();

            let x_sector = file_struc.x + -bounds.min_x;
            let z_sector = file_struc.z + -bounds.min_z;

            // for every pixel
            for x in 0..IMAGE_TILE_WIDTH {
                for z in 0..IMAGE_TILE_HEIGHT {
                    // get pixel from tile image
                    let pixel = tile_img.get_pixel(x.try_into().unwrap(), z.try_into().unwrap());

                    // calculate where pixel should go on output image
                    let output_pixel_x = x + (x_sector * IMAGE_TILE_WIDTH);
                    let output_pixel_z = z + (z_sector * IMAGE_TILE_HEIGHT);

                    output_imgbuf.put_pixel(
                        output_pixel_x.try_into().unwrap(),
                        output_pixel_z.try_into().unwrap(),
                        pixel,
                    )
                }
            }
        }

        output_imgbuf
    }

    // fn insert_image(
    //     souce_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    //     inserted_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    //     x_offset: u32,
    //     y_offset: u32,
    // ) {
    // }

    pub fn shrink_tiles(input_files: Vec<PathBuf>, output_dir: &str) {
        // clean output directory
        if PathBuf::from(output_dir).is_dir() {
            fs::remove_dir_all(output_dir).unwrap();
        }
        fs::create_dir(output_dir).unwrap();

        let mut filenums_map = HashMap::new();

        let mut rendered_output_tiles_map = HashMap::new();

        for file in &input_files {
            //> fill filename_and_numbers_vec
                let mut filename_and_numbers_vec: Vec<FilenameAndNumbers> = Vec::new();

                let file_name = file.file_stem().unwrap().to_str().unwrap();
                let split: Vec<&str> = file_name.split(',').collect();

                let x: i32 = split[0].parse().unwrap();
                let z: i32 = split[1].parse().unwrap();

                filename_and_numbers_vec.push(FilenameAndNumbers {
                    file_name: file.clone(),
                    x,
                    z,
                });

            //<> fill hashmap
                filenums_map.insert((x, z), file);
            //<
        }

        for ((x, y), _) in &filenums_map {
            // determine coords of output tile
            let output_tile_x = if *x < 0 { (*x - 1) / 2 } else { *x / 2 };
            let output_tile_y = if *y < 0 { (*y - 1) / 2 } else { *y / 2 };

            //skip any already rendered tiles
            if rendered_output_tiles_map.contains_key(&(output_tile_x, output_tile_y)) {
                continue;
            }

            // initialize output image
            let mut output_imgbuf = RgbaImage::new(
                (2 * IMAGE_TILE_WIDTH).try_into().unwrap(),
                (2 * IMAGE_TILE_HEIGHT).try_into().unwrap(),
            );

            //> convert 4 images into one big image
                // for the 4 sectors of the new tile
                for x_sector in 0..=1 {
                    for y_sector in 0..=1 {
                        let real_x = output_tile_x * 2 + x_sector;
                        let real_y = output_tile_y * 2 + y_sector;

                        match filenums_map.get(&(real_x, real_y)) {
                            Some(path) => {
                                let input_tile_img = image::open(&path).unwrap();

                                // transfer image
                                for x in 0..IMAGE_TILE_WIDTH {
                                    for y in 0..IMAGE_TILE_HEIGHT {
                                        // get pixel from tile image
                                        let pixel = input_tile_img
                                            .get_pixel(x.try_into().unwrap(), y.try_into().unwrap());

                                        // calculate where pixel should go on output image
                                        let output_pixel_x = x + (x_sector * IMAGE_TILE_WIDTH);
                                        let output_pixel_z = y + (y_sector * IMAGE_TILE_HEIGHT);

                                        output_imgbuf.put_pixel(
                                            output_pixel_x.try_into().unwrap(),
                                            output_pixel_z.try_into().unwrap(),
                                            pixel,
                                        )
                                    }
                                }
                            }
                            None => continue,
                        }
                    }
                }

            //<> resize output image
                let mut dynamic = DynamicImage::ImageRgba8(output_imgbuf);
                dynamic = dynamic.resize(
                    dynamic.dimensions().0 / 2,
                    dynamic.dimensions().1 / 2,
                    FilterType::Lanczos3,
                );

            //<> save file
                let output_tile_filename =
                    output_tile_x.to_string() + "," + &output_tile_y.to_string() + ".png";
                dynamic
                    .save(output_dir.to_owned() + &output_tile_filename)
                    .expect("failed to save file");
            //<

            // mark output image tile as rendered
            rendered_output_tiles_map.insert((output_tile_x, output_tile_y), true);
        }
    }

    pub fn image_to_tiles(image_path: &str, output_dir: &str) {
        // clear any existing output_dir
        if PathBuf::from(output_dir).is_dir() {
            fs::remove_dir_all(output_dir).unwrap();
        }
        fs::create_dir(output_dir).unwrap();

        let source_image = image::open(image_path).unwrap();
        let out_tile_width = 256;
        let out_tile_height = 256;

        let num_x_tiles = if source_image.width() % out_tile_width == 0 {
            source_image.width() / out_tile_width
        } else {
            (source_image.width() / out_tile_width) + 1
        };

        let num_y_tiles = if source_image.height() % out_tile_height == 0 {
            source_image.height() / out_tile_height
        } else {
            (source_image.height() / out_tile_height) + 1
        };

        let mut tile_image = RgbaImage::new(out_tile_width, out_tile_height);

        // for every sector in source image
        for sector_y in 0..num_y_tiles {
            for sector_x in 0..num_x_tiles {
                // for every pixel in new tile
                for y in 0..out_tile_height {
                    for x in 0..out_tile_width {
                        // calculate where pixel is in source image
                        let souce_x = out_tile_width * sector_x + x;
                        let souce_y = out_tile_width * sector_y + y;

                        let pixel =
                            if souce_x < source_image.width() && souce_y < source_image.height() {
                                source_image.get_pixel(souce_x, souce_y)
                            } else {
                                Rgba([0, 0, 0, 0])
                            };

                        tile_image.put_pixel(x, y, pixel)
                    }
                }

                // save file
                let output_tile_filename =
                    sector_x.to_string() + "," + &sector_y.to_string() + ".png";
                tile_image
                    .save(output_dir.to_owned() + &output_tile_filename)
                    .expect("failed to save file");
            }
        }
    }

    pub fn image_to_tiles_recursive(image_path: &str, output_dir: &str) {
        image_to_tiles(image_path, &(output_dir.to_owned() + "0/"));

        let mut count = 0;
        while get_files_in_dir(&(output_dir.to_owned() + &count.to_string() + "/"), "")
            .unwrap()
            .len()
            > 4
        {
            let files =
                get_files_in_dir(&(output_dir.to_owned() + &count.to_string() + "/"), "").unwrap();

            count += 1;

            shrink_tiles(files, &(output_dir.to_owned() + &count.to_string() + "/"));
        }
    }
}
