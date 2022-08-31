use colored::Colorize;
use image::GenericImageView;
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
    let image_path = "./input_images/cosmic_cliffs.png";
    let source_image = image::open(image_path).unwrap();
    clean_dir("./tiles/");
    image_to_tiles(
        image_path,
        (source_image.width() / 2).try_into().unwrap(),
        (source_image.height() / 2).try_into().unwrap(),
        "./tiles/0/",
    );
    generate_lods("./tiles/");
}
