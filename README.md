![output](https://github.com/user-attachments/assets/93ee1613-c2b2-4484-bfe1-66e762ee7b83)

# Eanray

Eanray is a simple Ray Tracer that converts a scene description (in a form of a script) into
a PPM file representing the rendered image.

## Features

Eanray's core is currently based on the ray tracer implementation from the
book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).
There is a plan to add more features in the future, but for now, the most relevant ones are the following:

* Ray-sphere intersection
* Color Shading
* Objects/Hittables
* Materials (Dielectrics, Lambertians, Metals)
* Antialising
* Depth of Field (Defocus Blur)
* Camera System
* Lua scripting for the Scene Descriptions

## How To Run

At the time of this writing, Eanray can only accept scene descriptions from the standard
input stream. To make it easier for users to write and modify scenes, it's recommended to
write them in files and feed the contents into Eanray:

```shell
$ caro run < <path-to-lua-script> --release
```

You should get an `output.ppm` that you can open with any image viewing program that
supports PPM.

### Examples

Here's the Lua script that describes the scene you probably already saw above in this
document: [cover.lua](examples/v0_1/cover.lua)

More examples can be found [here](examples), including some [scenes](examples/rt1w) from
[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).


## Technologies

The core renderer is written in [Rust](https://www.rust-lang.org/). Scene descriptions
provided by the should be written in [Lua](https://www.lua.org/)


