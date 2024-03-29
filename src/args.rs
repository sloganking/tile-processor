// use std::ffi::OsString;
use std::path::PathBuf;

use clap::Subcommand;

#[derive(Debug, clap::Parser)]
#[clap(version)]
pub struct Args {
    // /// The directory to read UTF-8 encoded text files from.
    // #[clap(long, short = 'i', default_value = "input", help_heading = "INPUT")]
    // pub input_dir: PathBuf,
    /// The various subcommands this program can run.
    #[clap(subcommand)]
    pub top_commands: TopSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum TopSubcommands {
    /// Slices an image into image tiles.
    GenTiles(GenTilesArgs),
    /// Slices an image into image tiles and generates tile LOD layers.
    GenTileLayers(GenTilesArgs),
    /// Creates single image from directory of tiles.
    StitchImage(StitchImageArgs),
    /// Generates layers from directory of tiles. The existing tiles will be moved into a "./0/" folder.
    /// subsuquent layers will be stored in neighboring folders.
    TilesToLayers(TilesToLayersArgs),
}

#[derive(Debug, Clone, clap::Parser)]
pub struct GenTilesArgs {
    /// The image to generate tiles from.
    #[clap(long, short = 'i', help_heading = "IO")]
    pub input: PathBuf,

    /// The directory to save generated tiles to.
    #[clap(long, short = 'o', help_heading = "IO")]
    pub output: PathBuf,

    /// The width and height (in pixels) of output tiles.
    #[clap(long, default_value_t = 256, help_heading = "IO")]
    pub tile_dimensions: u32,

    /// The x pixel to make tile pixel 0,0.
    #[clap(long, help_heading = "IO")]
    pub x_offset: Option<i32>,

    /// The y pixel to make tile pixel 0,0.
    #[clap(long, help_heading = "IO")]
    pub y_offset: Option<i32>,
}

#[derive(Debug, clap::Parser)]
pub struct StitchImageArgs {
    /// The directory of tiles to turn into an image.
    #[clap(long, short = 'i')]
    pub input: PathBuf,

    /// The location to save the outputed image to.
    #[clap(long, short = 'o')]
    pub output: PathBuf,
}

#[derive(Debug, clap::Parser)]
pub struct TilesToLayersArgs {
    /// The directory of tiles to generate layers from.
    #[clap(long, short = 'i')]
    pub input: PathBuf,
}
