use colored::Colorize;
use image::{io::Reader, GenericImageView};

mod args;
use args::{GenTilesArgs, TopSubcommands};
use map_combine::tiler::*;

fn _print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
}

fn main() {
    let args: args::Args = clap::Parser::parse();

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

    println!("args: {:?}", args);

    match args.top_commands {
        TopSubcommands::GenTiles(gen_tiles_args) => {
            // get input image dimensions
            let image_path = &gen_tiles_args.input.into_os_string().into_string().unwrap();
            let dimensions = Reader::open(image_path).unwrap().into_dimensions().unwrap();

            let output_dir = gen_tiles_args
                .output
                .into_os_string()
                .into_string()
                .unwrap();

            println!("cleaning dir...");
            clean_dir(&output_dir);

            println!("slicing tiles...");
            image_to_tiles(
                image_path,
                gen_tiles_args
                    .x_offset
                    .unwrap_or_else(|| (dimensions.0 / 2).try_into().unwrap()),
                gen_tiles_args
                    .y_offset
                    .unwrap_or_else(|| (dimensions.1 / 2).try_into().unwrap()),
                &output_dir,
                gen_tiles_args.tile_dimensions,
            );
        }
        TopSubcommands::GenTileLayers => todo!(),
        TopSubcommands::StitchImage => {
            todo!()
        }
    }

    // // turn image into tiles and LODs
    // let image_path = "./input_images/cosmic_cliffs.png";
    // let source_image = image::open(image_path).unwrap();
    // clean_dir("./tiles/");
    // image_to_tiles(
    //     image_path,
    //     (source_image.width() / 2).try_into().unwrap(),
    //     (source_image.height() / 2).try_into().unwrap(),
    //     "./tiles/0/",
    // );
    // generate_lods("./tiles/");
}
