use colored::Colorize;
use image::{io::Reader, GenericImageView};
use map_combine::tiler::*;

fn _print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
}

fn main() {
    //> turn tiles into one image
        // let files = get_files_in_dir("./input", "").unwrap();

        // if files.is_empty() {
        //     print_err("no files found");
        //     return;
        // }

        // let output_imgbuf = consolidate_images(&files);

        // Write the contents of this image to the Writer in PNG format.
        // output_imgbuf
        //     .save("./0th.png")
        //     .expect("failed to save file");
    //<

    // turn image into tiles and LODs
    let image_path = "./input_images/androm.png";
    let dimensions = Reader::open(image_path).unwrap().into_dimensions().unwrap();

    println!("dimensions: {:?}", dimensions);
    clean_dir("./tiles/");
    image_to_tiles(
        image_path,
        (dimensions.0/2).try_into().unwrap(),
        (dimensions.1/2).try_into().unwrap(),
        "./tiles/0/",
    );
    generate_lods("./tiles/");
}
