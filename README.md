# Midpoint Circle Visualization Algorithm - In Rust 

- It uses [Netpbm](https://en.wikipedia.org/wiki/Netpbm) library as the drawing
  canvas specifically .ppm file format with P6 magic number and 256 color-support.(24-bit
  per pixel, 8-RED, 8-BLUE, 8-GREEN).

  - Because it uses ppm, there is no external depedency and hence no use of
    package manager (cargo), rust-compiler does the work.
    
-  There are 3 patterns in this : striped, checkered & solid circle for now,
   uncomment the code accordingly to see different patterns.
   
TODO:
1. On low enough resolutions, this algo results in pixelation, try to better it,
   make it better. 
   
## References:
This is ~~almost~~ an exact clone of the stream done by
[tsoding](https://www.youtube.com/c/TsodingDaily), used purely as a learning
experience with the hope to implement some of my own patterns/ use some other
efficient algo to make it better. 
