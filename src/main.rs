use std::{fs, path::Path};

use colored::Colorize;
use image::io::Reader;

mod args;
use args::{GenTilesArgs, TopSubcommands};
use tileproc::tiler::*;

fn print_err(err: &str) -> ! {
    println!("{}: {}", "error".red().bold(), err);
    std::process::exit(1);
}

fn gen_tiles_to_dir(gen_tiles_args: &GenTilesArgs) {
    // get input image dimensions
    let dimensions = Reader::open(&gen_tiles_args.input)
        .unwrap()
        .into_dimensions()
        .unwrap();

    println!("cleaning dir...");
    clean_dir(&gen_tiles_args.output);

    image_to_tiles_tile_cache(
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

/// Moves all files (not directories) inside one directory, into another directory. Does not delete the directory that files were moved from.
/// Does not move files between drives.
fn move_files_in_directory(from: &Path, to: &Path) {
    for entry in fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() {
            let filename = path.file_name().unwrap();

            let mut new_to = to.to_path_buf();
            new_to.push(filename);

            std::fs::rename(&path, new_to).unwrap();
        }
    }
}

fn main() {
    for i in (-1000..0).rev() {
        println!("i: {} --- (i % 256): {:?}", i, to_tile_location(i, 0));
    }

    let args: args::Args = clap::Parser::parse();

    match args.top_commands {
        TopSubcommands::GenTiles(gen_tiles_args) => {
            gen_tiles_to_dir(&gen_tiles_args);
        }
        TopSubcommands::GenTileLayers(gen_tiles_args) => {
            clean_dir(&gen_tiles_args.output);

            let mut new_gen_tiles_args = gen_tiles_args.clone();
            new_gen_tiles_args.output.push("0/");
            gen_tiles_to_dir(&new_gen_tiles_args);

            generate_lods(&gen_tiles_args.output);
        }
        TopSubcommands::StitchImage(stitch_image_args) => {
            // Assertions
            if !stitch_image_args.input.is_dir() {
                print_err("input is not a directory.");
            }
            stitch_image_args.output.extension().unwrap_or_else(|| {
                print_err("output has no file extension.");
            });

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
        TopSubcommands::TilesToLayers(tiles_to_layers_args) => {
            if !tiles_to_layers_args.input.is_dir() {
                print_err("input is not a directory.");
            }
            let zero_path = tiles_to_layers_args.input.join("0/");
            clean_dir(&zero_path);
            move_files_in_directory(&tiles_to_layers_args.input, &zero_path);
            generate_lods(&tiles_to_layers_args.input);
        }
    }
}
