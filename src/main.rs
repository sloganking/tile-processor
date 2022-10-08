use colored::Colorize;
use image::io::Reader;

mod args;
use args::TopSubcommands;
use map_combine::tiler::*;

fn print_err(err: &str) {
    println!("{}: {}", "error".red().bold(), err);
    std::process::exit(1);
}

fn main() {
    let args: args::Args = clap::Parser::parse();

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
        TopSubcommands::GenTileLayers(gen_tiles_args) => {
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
                &(output_dir.clone() + "0/"),
                gen_tiles_args.tile_dimensions,
            );

            generate_lods(&output_dir);
        }
        TopSubcommands::StitchImage(stitch_image_args) => {
            // Assertions
            if !stitch_image_args.input.is_dir() {
                print_err("input is not a directory.");
            }
            // if !stitch_image_args.output.is_file(){
            //     print_err("output is not a file.");
            // }

            let files = get_files_in_dir(
                &stitch_image_args
                    .input
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                "",
            )
            .unwrap();

            if files.is_empty() {
                print_err("no files found in input directory.");
            }

            let output_imgbuf = consolidate_images(&files);

            // Write the contents of this image to the Writer in PNG format.
            output_imgbuf
                .save(stitch_image_args.output)
                .expect("failed to save file");
        }
    }
}
