<img width="1200" height="675" alt="planets_on_ground8" src="https://github.com/user-attachments/assets/bea42041-ab0d-447a-92a5-1b69daef3ef9" />

# Eanray

Eanray is a simple Ray Tracer, currently being written in Rust, that converts a Lua script describing the scene into a 
PPM file representing the rendered image.

## Features

There is a plan to add more features in the future, but for now, the most relevant ones are the following:

* Ray-object intersection
  * Primitives: Spheres, Planar (Triangles, Quadrilaterals)
* Color Shading
* Objects/Hittables
* Materials (Dielectrics, Lambertians, Metals)
* Antialising
* Depth of Field (Defocus Blur)
* Configurable Camera System
* Lua scripting for the Scene Descriptions
* Motion Blur
* Bounding Volume Hierarchy
* Texture Mapping
* Perlin Noise
* Light Sources
* Diagnostics (Metrics, Statistics)

## How To Run

At the time of this writing, Eanray can only accept Lua scripts as scene descriptions. To provide the Lua script you
want to render, you need to pass its path as a command line argument:

```shell
$ caro run <path-to-lua-script> --release
```

or `RUST_LOG=info caro run <lua-script> --release` if you want to set the logging level to `INFO`.

You should get an `output.ppm` that you can open with any image viewing program that
supports PPM.

Note: Instructions on running executables will be provided after the first release. 

### Examples

Here's the Lua script that describes the scene you probably already saw above in this
document: [cover.lua](examples/v0_1/cover.lua)

More examples can be found [here](examples), including some [scenes](examples/rt1w) from
[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).


## Technologies

The core renderer is written in [Rust](https://www.rust-lang.org/). Scene descriptions
provided by the user should be written in [Lua](https://www.lua.org/)

## References and Resources

[Ray Tracing in One Weekend](https://raytracing.github.io/) series by Peter Shirley et al. 

[Here](examples/rt1w/scene23_final_scene.lua) is the Lua script that renders the Final Scene from the first book.

The result should look like the following image, except for the changes in the small spheres' positions and materials caused by randomization:

![rtnw_image_1](https://github.com/user-attachments/assets/95cc6833-c36d-4dd0-a1f9-4410d9eaeda7)


## Assets

Planet textures by [Solar System Scope](https://www.solarsystemscope.com/textures/), licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).


