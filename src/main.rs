use colored::Colorize;
use glob::{glob, GlobError};
use image::{GenericImageView, Rgb, RgbImage, Rgba, RgbaImage};
use std::{fs, path::PathBuf};

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
    fileName: PathBuf,
    x: i32,
    z: i32,
}

fn main() {
    println!("Hello, world!");

    let files = get_files_in_dir("./input", "").unwrap();

    if files.len() == 0 {
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
        println!("{}", file.file_stem().unwrap().to_str().unwrap());

        let file_name = file.file_stem().unwrap().to_str().unwrap();
        let split: Vec<&str> = file_name.split(",").collect();

        let x: i32 = split[0].parse().unwrap();
        let z: i32 = split[1].parse().unwrap();

        filename_and_numbers_vec.push(FilenameAndNumbers {
            fileName: file.clone(),
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

    println!("{:?}", bounds);

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
        let tile_img = image::open(&file_struc.fileName).unwrap();

        let x_sector = (file_struc.x + -bounds.min_x);
        let z_sector = (file_struc.z + -bounds.min_z);

        println!("file_struc.x: {}", file_struc.x);
        println!("file_struc.z: {}", file_struc.z);

        println!("x_sector: {}", x_sector);
        println!("z_sector: {}", z_sector);

        // for every pixel
        for x in 0..image_tile_width {
            for z in 0..image_tile_height {
                // get pixel from tile image
                let pixel = tile_img.get_pixel(x.try_into().unwrap(), z.try_into().unwrap());

                // calculate where pixel should go on output image

                let output_pixel_x = x + (x_sector * image_tile_width);

                let output_pixel_z = z + (z_sector * image_tile_height);

                // let output_pixel_x = x - (-bounds.min_x * image_tile_width);
                // let output_pixel_z = z - (-bounds.min_z * image_tile_height);

                // println!("output_pixel_x: {}",output_pixel_x);
                // println!("output_pixel_z: {}",output_pixel_z);

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

    let img = image::open(&files[0]).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // let x_offset = (-bounds.max_x + bounds.min_x.abs()) -1;

    println!("x_offset: {}", -bounds.min_x * image_tile_width);

    let pixel = Rgba([0, 0, 0, 0]);

    // positive Z is South.
    // positive X is East
}
