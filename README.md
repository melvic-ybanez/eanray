<img width="1200" height="675" alt="planets_on_ground2" src="https://github.com/user-attachments/assets/3d4a1ab1-eb82-4d01-8c17-d2f1ffb50cc9" />

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
* Diagnostics (Metrics, Statistics)

## How To Run

At the time of this writing, Eanray can only accept scene descriptions from the standard
input stream. To make it easier for users to write and modify scenes, it's recommended to
write them in files and feed the contents into Eanray:

```shell
$ caro run <lua-script> --release
```

or `RUST_LOG=info caro run <lua-script> --release` if you want to set the logging level to `INFO`.

You should get an `output.ppm` that you can open with any image viewing program that
supports PPM.

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

Planet textures by Solar System Scope, licensed under CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/)


