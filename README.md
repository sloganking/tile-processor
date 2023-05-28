# tile-processor

`tile-processor` is a utility for turning large images into tiles, generating tile LODs, or turning tiles back into one image. 
Beyond a certain resolution, images may not be viewable due to exhausting RAM, or being too large to render at a reasonable FPS.
This is a problem fixed by tile image viewers. This repository does the work of prepairing tiles for those viewers.


An example of how a large image can be rendered at various levels of detail (LOD)
![](https://raw.githubusercontent.com/banesullivan/localtileserver/main/imgs/tile-diagram.gif)
