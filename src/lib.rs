pub mod args;

pub mod tiler {
    use glob::{glob, GlobError};
    use image::{
        imageops::FilterType, io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgba,
        RgbaImage,
    };
    use std::{
        collections::HashMap,
        fs, io,
        path::{Path, PathBuf},
    };

    use crate::args::GenTilesArgs;

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

        // filter out directories
        let paths = paths.into_iter().filter(|e| e.is_file());

        let paths: Vec<PathBuf> = paths.into_iter().collect();
        Ok(paths)
    }

    /// Stitches image tiles into one image
    pub fn consolidate_images(files: &[PathBuf]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        // get initial tile dimensions
        let mut tile_dimensions: (u32, u32) = (0, 0);
        let source_image = image::open(&files[0]).unwrap();
        tile_dimensions.0 = source_image.width();
        tile_dimensions.1 = source_image.height();
        let tile_dimensions = tile_dimensions;

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
            (xdiff * tile_dimensions.0 as i32).try_into().unwrap(),
            (zdiff * tile_dimensions.1 as i32).try_into().unwrap(),
        );

        for file_struc in &filename_and_numbers_vec {
            let tile_img = image::open(&file_struc.file_name).unwrap();

            let x_sector = file_struc.x + -bounds.min_x;
            let z_sector = file_struc.z + -bounds.min_z;

            // for every pixel
            for x in 0..tile_dimensions.0 as i32 {
                for z in 0..tile_dimensions.1 as i32 {
                    // get pixel from tile image
                    let pixel = tile_img.get_pixel(x.try_into().unwrap(), z.try_into().unwrap());

                    // calculate where pixel should go on output image
                    let output_pixel_x = x + (x_sector * tile_dimensions.0 as i32);
                    let output_pixel_z = z + (z_sector * tile_dimensions.1 as i32);

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

    /// remove contents inside a directory, without deleting the directory itself.
    fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let path = entry.unwrap().path();

            if path.is_file() {
                fs::remove_file(path)?;
            } else if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                panic!("https://i.kym-cdn.com/entries/icons/original/000/013/306/2dd.jpg")
            }
        }
        Ok(())
    }

    /// Erases all content of an existing directory, or creates an empty new one.
    pub fn clean_dir(path: &Path) {
        // clear any existing output_dir
        if path.is_dir() {
            remove_dir_contents(path).unwrap();
        } else {
            fs::create_dir(path).unwrap();
        }
    }

    /// Compresses one lod layer
    pub fn shrink_tiles(input_files: Vec<PathBuf>, output_dir: &Path) {
        // cancel if nothing to do
        if input_files.is_empty() {
            return;
        }

        clean_dir(output_dir);

        let mut filenums_map = HashMap::new();
        let mut rendered_output_tiles_map = HashMap::new();

        // get initial tile dimensions
        let mut tile_dimensions: (u32, u32) = (0, 0);
        let source_image = image::open(&input_files[0]).unwrap();
        tile_dimensions.0 = source_image.width();
        tile_dimensions.1 = source_image.height();
        let tile_dimensions = tile_dimensions;

        for file in &input_files {
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

            filenums_map.insert((x, z), file);
        }

        let filenums_map = filenums_map;

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())
            .build()
            .unwrap();

        pool.scope(|s| {
            for (x, y) in filenums_map.keys() {
                // determine coords of output tile
                let output_tile_x = if *x < 0 { (*x - 1) / 2 } else { *x / 2 };
                let output_tile_y = if *y < 0 { (*y - 1) / 2 } else { *y / 2 };

                //skip any already rendered tiles
                if rendered_output_tiles_map.contains_key(&(output_tile_x, output_tile_y)) {
                    continue;
                }

                // mark output image tile as rendered
                rendered_output_tiles_map.insert((output_tile_x, output_tile_y), true);

                let test_closure = |output_tile_x: i32, output_tile_y: i32| {
                    let output_tile_x = output_tile_x;
                    let output_tile_y = output_tile_y;

                    // initialize output image
                    let mut output_imgbuf =
                        RgbaImage::new(2 * tile_dimensions.0, 2 * tile_dimensions.1);

                    // convert 4 images into one big image
                    // for the 4 sectors of the new tile
                    for x_sector in 0..=1 {
                        for y_sector in 0..=1 {
                            let real_x = output_tile_x * 2 + x_sector;
                            let real_y = output_tile_y * 2 + y_sector;

                            match &filenums_map.get(&(real_x, real_y)) {
                                Some(path) => {
                                    let input_tile_img = image::open(&path).unwrap();

                                    // transfer image
                                    for x in 0..tile_dimensions.0 {
                                        for y in 0..tile_dimensions.1 {
                                            // get pixel from tile image
                                            let pixel = input_tile_img.get_pixel(x, y);

                                            // calculate where pixel should go on output image
                                            let output_pixel_x =
                                                x as i32 + (x_sector * tile_dimensions.0 as i32);
                                            let output_pixel_z =
                                                y as i32 + (y_sector * tile_dimensions.1 as i32);

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

                    // resize output image
                    let mut dynamic = DynamicImage::ImageRgba8(output_imgbuf);
                    dynamic = dynamic.resize(
                        dynamic.dimensions().0 / 2,
                        dynamic.dimensions().1 / 2,
                        FilterType::Lanczos3,
                    );

                    // save file
                    let output_tile_filename =
                        output_tile_x.to_string() + "," + &output_tile_y.to_string() + ".png";
                    dynamic
                        .save(output_dir.join(&output_tile_filename))
                        .expect("failed to save file");
                };

                s.spawn(move |_| test_closure(output_tile_x, output_tile_y));
            }
        });
    }

    /// converts an image into square image tiles.
    pub fn image_to_tiles(
        image_path: &Path,
        x_offset: i32,
        y_offset: i32,
        output_dir: &Path,
        tile_dimensions: u32,
    ) {
        clean_dir(output_dir);

        println!("decoding image...");
        let source_image = image::open(image_path).unwrap();
        let out_tile_width = tile_dimensions;
        let out_tile_height = tile_dimensions;

        println!("slicing tiles...");

        let (top_left_sector, bottom_right_sector) = get_limit_sectors(
            x_offset,
            y_offset,
            (out_tile_width as f32, out_tile_height as f32),
            (source_image.width() as f32, source_image.height() as f32),
        );

        std::thread::scope(|s| {
            // for every sector in source image
            for sector_y in top_left_sector.1..=bottom_right_sector.1 {
                for sector_x in top_left_sector.0..=bottom_right_sector.0 {
                    let mut tile_image = RgbaImage::new(out_tile_width, out_tile_height);
                    let mut tile_empty = true;

                    // for every pixel in new tile
                    for y in 0..out_tile_height as i32 {
                        for x in 0..out_tile_width as i32 {
                            // calculate where pixel is in source image
                            let souce_x = (out_tile_width as i32 * sector_x + x) + x_offset;
                            let souce_y = (out_tile_width as i32 * sector_y + y) + y_offset;

                            let pixel = if souce_x >= 0
                                && souce_x < source_image.width().try_into().unwrap()
                                && souce_y >= 0
                                && souce_y < source_image.height().try_into().unwrap()
                            {
                                let pixel = source_image.get_pixel(
                                    souce_x.try_into().unwrap(),
                                    souce_y.try_into().unwrap(),
                                );

                                if pixel != Rgba([0, 0, 0, 0]) {
                                    tile_empty = false;
                                }

                                pixel
                            } else {
                                Rgba([0, 0, 0, 0])
                            };

                            tile_image.put_pixel(
                                x.try_into().unwrap(),
                                y.try_into().unwrap(),
                                pixel,
                            )
                        }
                    }

                    let file_save_closure =
                        |sector_x: i32,
                         sector_y: i32,
                         tile_image: ImageBuffer<Rgba<u8>, Vec<u8>>| {
                            let output_tile_filename =
                                sector_x.to_string() + "," + &sector_y.to_string() + ".png";
                            tile_image
                                .save(output_dir.join(output_tile_filename))
                                .expect("failed to save file");
                        };

                    // save file
                    if !tile_empty {
                        s.spawn(move || file_save_closure(sector_x, sector_y, tile_image));
                    }
                }
            }
        });
    }

    /// Generates LOD layers
    ///
    /// LOD layers are generated by compressing 4 pixels into one
    ///
    /// LOD layers will be generated until the most recent one consists of 4 tiles or less
    ///
    /// Something like https://raw.githubusercontent.com/banesullivan/localtileserver/main/imgs/tile-diagram.gif
    pub fn generate_lods(output_dir: &Path) {
        let mut count = 0;
        let mut files: Vec<PathBuf>;
        while {
            // get num files in dir
            let dirs = fs::read_dir(output_dir.join(count.to_string())).unwrap();
            files = dirs.map(|dir| dir.unwrap().path()).collect();
            files.len() > 4
        } {
            count += 1;
            shrink_tiles(files, &output_dir.join(count.to_string()));
        }
    }

    pub fn sector_at_pos(x: f32, y: f32, tile_dimensions: (f32, f32)) -> (i32, i32) {
        let two: f32 = 2.0;
        let lod = 0;
        // let screen_point_coords = screen_pos_to_coord(x, y, camera);

        let screen_point_coords = (x, y);

        // get sector x
        let tile_world_x_size = tile_dimensions.0 as f32 * two.powf(lod as f32);
        let screen_point_sector_x = if screen_point_coords.0 < 0.0 {
            (screen_point_coords.0 / tile_world_x_size) as i32 - 1
        } else {
            (screen_point_coords.0 / tile_world_x_size) as i32
        };

        // get sector y
        let tile_world_y_size = tile_dimensions.1 as f32 * two.powf(lod as f32);
        let screen_point_sector_y = if screen_point_coords.1 < 0.0 {
            (screen_point_coords.1 / tile_world_y_size) as i32 - 1
        } else {
            (screen_point_coords.1 / tile_world_y_size) as i32
        };

        (screen_point_sector_x, screen_point_sector_y)
    }

    pub fn get_limit_sectors(
        x_offset: i32,
        y_offset: i32,
        tile_dimensions: (f32, f32),
        image_dimensions: (f32, f32),
    ) -> ((i32, i32), (i32, i32)) {
        let top_left_sector =
            sector_at_pos(0. - x_offset as f32, 0. - y_offset as f32, tile_dimensions);

        let bottom_right_sector = sector_at_pos(
            image_dimensions.0 - x_offset as f32,
            image_dimensions.1 - y_offset as f32,
            tile_dimensions,
        );

        (top_left_sector, bottom_right_sector)
    }

    pub fn gen_tiles_to_dir(gen_tiles_args: &GenTilesArgs) {
        // get input image dimensions
        let dimensions = Reader::open(&gen_tiles_args.input)
            .unwrap()
            .into_dimensions()
            .unwrap();

        println!("cleaning dir...");
        clean_dir(&gen_tiles_args.output);

        image_to_tiles(
            &gen_tiles_args.input,
            gen_tiles_args
                .x_offset
                .unwrap_or_else(|| (dimensions.0 / 2).try_into().unwrap()),
            gen_tiles_args
                .y_offset
                .unwrap_or_else(|| (dimensions.1 / 2).try_into().unwrap()),
            &gen_tiles_args.output,
            gen_tiles_args.tile_dimensions,
        );
    }
}
