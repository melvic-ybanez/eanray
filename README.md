![Sun Campfire 6](https://github.com/user-attachments/assets/e69a6d3e-3032-43c6-9b3e-1851e6196622)

# Eanray

Eanray is a Ray Tracer that converts a scene description into a PPM file representing the rendered image.

## Features

There is a plan to add more features in the future, but for now, the most relevant ones are the following:

* Optimizations:
  * Bounding Volume Hierarchy
  * Multithreaded Rendering (Tile-based)
* Naive Monte Carlo Global Illumination
* Ray-object intersection
  * Primitives: Quadrics (Spheres, Cylinders, Cones), Boxes, Constant Medium, Planar (Triangles, Quadrilaterals, Disks), Planes
* Materials: Dielectrics, Lambertians, Metals, DiffuseLight, Isotropic
* Antialising
* Depth of Field (Defocus Blur)
* Configurable Camera System
* Lua scripting for the Scene Descriptions
* Motion Blur (currently supported only for Spheres)
* Texture Mappings: Checkers, Perlin Noise, Images
* Light Sources
* Transformations: Translation, Rotations, Scaling
* Diagnostics: Metrics, Statistics

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
document: [sun_campfire.lua](examples/v2025_09/sun_campfire.lua). 
You can modify the script to lower the SPP for testing/debugging purposes.

 Another example you can try is [this one](examples/v2025_09/planets_on_the_ground.lua):

![Planets on the ground](https://github.com/user-attachments/assets/1730eb19-2679-4866-b18e-01c740fd1768)

More examples can be found [here](examples), including some [scenes](examples/rt1w) from
[Ray Tracing in One Weekend](https://raytracing.github.io) series.


## Technologies

The core renderer is written in [Rust](https://www.rust-lang.org/). Scene descriptions
provided by the user should be written in [Lua](https://www.lua.org/)

## References and Resources

[Ray Tracing in One Weekend](https://raytracing.github.io/) series by Peter Shirley et al. 

[The Ray Tracer Challenge](http://raytracerchallenge.com/) by Jamis Buck


## Assets

Planet textures by [Solar System Scope](https://www.solarsystemscope.com/textures/), licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).


