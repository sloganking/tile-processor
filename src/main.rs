use colored::Colorize;
use glob::{glob, GlobError};
use image::{imageops::FilterType, DynamicImage, GenericImageView, RgbaImage};
use std::path::PathBuf;

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

fn print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
}

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

fn main() {
    let files = get_files_in_dir("./input", "").unwrap();

    if files.is_empty() {
        print_err("no files found");
        return;
    }

    let mut bounds = Bounds {
        max_x: i32::MIN,
        max_z: i32::MIN,
        min_x: i32::MAX,
        min_z: i32::MAX,
    };

    let mut filename_and_numbers_vec: Vec<FilenameAndNumbers> = Vec::new();

    // find max and min dimensions
    for file in &files {
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

    let image_tile_width = 512;
    let image_tile_height = 512;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut output_imgbuf = RgbaImage::new(
        (xdiff * image_tile_width).try_into().unwrap(),
        (zdiff * image_tile_height).try_into().unwrap(),
    );

    for file_struc in &filename_and_numbers_vec {
        let tile_img = image::open(&file_struc.file_name).unwrap();

        let x_sector = file_struc.x + -bounds.min_x;
        let z_sector = file_struc.z + -bounds.min_z;

        // for every pixel
        for x in 0..image_tile_width {
            for z in 0..image_tile_height {
                // get pixel from tile image
                let pixel = tile_img.get_pixel(x.try_into().unwrap(), z.try_into().unwrap());

                // calculate where pixel should go on output image
                let output_pixel_x = x + (x_sector * image_tile_width);
                let output_pixel_z = z + (z_sector * image_tile_height);

                output_imgbuf.put_pixel(
                    output_pixel_x.try_into().unwrap(),
                    output_pixel_z.try_into().unwrap(),
                    pixel,
                )
            }
        }
    }

    // Write the contents of this image to the Writer in PNG format.
    output_imgbuf.save("./test.png").unwrap();

    //>  shrink image a few times
        let mut dynamic = DynamicImage::ImageRgba8(output_imgbuf);

        dynamic = dynamic.resize(
            dynamic.dimensions().0 / 2,
            dynamic.dimensions().1 / 2,
            FilterType::Lanczos3,
        );
        dynamic.save("./test2.png").unwrap();

        dynamic = dynamic.resize(
            dynamic.dimensions().0 / 2,
            dynamic.dimensions().1 / 2,
            FilterType::Lanczos3,
        );
        dynamic.save("./test3.png").unwrap();

        dynamic = dynamic.resize(
            dynamic.dimensions().0 / 2,
            dynamic.dimensions().1 / 2,
            FilterType::Lanczos3,
        );
        dynamic.save("./test4.png").unwrap();
    //<
}
