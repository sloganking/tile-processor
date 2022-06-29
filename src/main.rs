use colored::Colorize;
use map_combine::tiler::*;

fn _print_err(err: &str) {
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

    // turn image into tiles and LODs
    clean_dir("./tiles/");
    image_to_tiles("./input_images/cat.png", 0, 0, "./tiles/0/");
    generate_lods("./tiles/");
}
