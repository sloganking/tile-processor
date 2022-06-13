use colored::Colorize;
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use map_combine::tiler::{
    consolidate_images, get_files_in_dir, image_to_tiles, image_to_tiles_recursive, shrink_tiles,
};

fn print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
}

fn main() {
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

    //> shrink image a few times
        // let mut dynamic = DynamicImage::ImageRgba8(output_imgbuf);

        // dynamic = dynamic.resize(
        //     dynamic.dimensions().0 / 2,
        //     dynamic.dimensions().1 / 2,
        //     FilterType::Lanczos3,
        // );
        // dynamic.save("./4th.png").expect("failed to save file");

        // dynamic = dynamic.resize(
        //     dynamic.dimensions().0 / 2,
        //     dynamic.dimensions().1 / 2,
        //     FilterType::Lanczos3,
        // );
        // dynamic.save("./8th.png").expect("failed to save file");

        // dynamic = dynamic.resize(
        //     dynamic.dimensions().0 / 2,
        //     dynamic.dimensions().1 / 2,
        //     FilterType::Lanczos3,
        // );
        // dynamic.save("./16th.png").expect("failed to save file");
    //<

    //> shrink test
        // shrink_tiles(files, "./output_tiles/");

        // let files = get_files_in_dir("./output_tiles", "").unwrap();
        // if files.is_empty() {
        //     print_err("no files found");
        //     return;
        // }

        // let output_imgbuf = consolidate_images(&files);

        // // Write the contents of this image to the Writer in PNG format.
        // output_imgbuf
        //     .save("./shrink_test.png")
        //     .expect("failed to save file");
    //<

    image_to_tiles_recursive("cat.png", "./tiles/");
}
