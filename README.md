# tile-processor

`tile-processor` is a utility for turning large images into tiles, generating tile LODs, or turning tiles back into one image. 
Beyond a certain resolution, images may not be viewable due to exhausting RAM, or being too large to render at a reasonable FPS.
This is a problem fixed by tile image viewers. This repository does the work of converting images into tiles for those viewers.

An example of how a large image can be rendered at various levels of detail.
![](https://raw.githubusercontent.com/banesullivan/localtileserver/main/imgs/tile-diagram.gif)

## Usage
```
Usage: tileproc <COMMAND>

Commands:
  gen-tiles        Slices an image into image tiles
  gen-tile-layers  Slices an image into image tiles and generates tile LOD layers
  stitch-image     Creates single image from directory of tiles
  tiles-to-layers  Generates layers from directory of tiles. The existing tiles will be moved into a "./0/" folder. subsuquent layers will be stored in neighboring folders
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
