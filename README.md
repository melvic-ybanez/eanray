<img width="1200" height="675" alt="palnets_on_ground10" src="https://github.com/user-attachments/assets/ab464ff3-e0b6-479d-8496-8048ebd433bb" />

# Eanray

Eanray is a simple Ray Tracer, currently being written in Rust, that converts a Lua script describing the scene into a 
PPM file representing the rendered image.

## Features

There is a plan to add more features in the future, but for now, the most relevant ones are the following:

* Multithreaded Rendering
* Ray-object intersection
  * Primitives: Spheres, Planar (Triangles, Quadrilaterals, Boxes, Constant Medium, Disks)
* Global Illumination
* Materials (Dielectrics, Lambertians, Metals, DiffuseLight, Isotropic)
* Antialising
* Depth of Field (Defocus Blur)
* Configurable Camera System
* Lua scripting for the Scene Descriptions
* Motion Blur (currently supported only for Spheres)
* Bounding Volume Hierarchy
* Texture Mapping (Checkers, Perlin Noise, Images)
* Light Sources
* Transformations (Translation, Rotation-Y)
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

[Here](examples/rtnw/scene10_final_scene.lua) is the Lua script for Peter Shirley's "Sweet Dreams", the final image of the second book. 
If you feed that to Eanray, you'll get the image below:

![sweet_dreams](https://github.com/user-attachments/assets/d34f8454-a3e0-4142-b636-547188c1ad2e)


## Assets

Planet textures by [Solar System Scope](https://www.solarsystemscope.com/textures/), licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).


