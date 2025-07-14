<img width="1200" height="675" alt="planets_on_ground6" src="https://github.com/user-attachments/assets/d41e347b-2421-4722-9646-617c2f277193" />

# Eanray

Eanray is a simple Ray Tracer that converts a scene description (in a form of a script) into
a PPM file representing the rendered image.

## Features

There is a plan to add more features in the future, but for now, the most relevant ones are the following:

* Ray-sphere intersection
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
* Diagnostics (Metrics, Statistics)

## How To Run

At the time of this writing, Eanray can only accept Lua scripts as scene descriptions. To provide the Lua script you
want to render, you need to pass its path as a command line argument:

```shell
$ caro run <lua-script> --release
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

![rtnw_image_1](https://github.com/user-attachments/assets/95cc6833-c36d-4dd0-a1f9-4410d9eaeda7)

[Ray Tracing in One Weekend](https://raytracing.github.io/) series by Peter Shirley et al.  

## Assets

Planet textures by Solar System Scope, licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)


