![Planets on the ground](https://github.com/user-attachments/assets/1730eb19-2679-4866-b18e-01c740fd1768)

# Eanray

Eanray is a simple Ray Tracer, currently being written in Rust, that converts a Lua script describing a 3D scene into a 
PPM file representing the rendered image.

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
* Transformations: Translation, Rotations
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
document: [cover.lua](examples/v2025_09/cover.lua)

 Another example you can try is [this one](examples/v2025_09/sun_campfire.lua) (which might take longer to render due to 
 a higher SPP). Running the script yields:

![Sun Campire](https://github.com/user-attachments/assets/405458a1-d53e-4ea2-8aa2-a7550a4046a5)

You can modify the script to lower the SPP for testing/debugging purposes.

More examples can be found [here](examples), including some [scenes](examples/rt1w) from
[Ray Tracing in One Weekend](https://raytracing.github.io) series.


## Technologies

The core renderer is written in [Rust](https://www.rust-lang.org/). Scene descriptions
provided by the user should be written in [Lua](https://www.lua.org/)

## References and Resources

[Ray Tracing in One Weekend](https://raytracing.github.io/) series by Peter Shirley et al. 

[Here](examples/rtnw/scene10_final_scene.lua) is the Lua script for Peter Shirley's ["Sweet Dreams"](https://raytracing.github.io/books/RayTracingTheNextWeek.html#ascenetestingallnewfeatures), the final image of the [second book](https://raytracing.github.io/books/RayTracingTheNextWeek.html). 
If you feed that to Eanray, you'll get the image below:

![sweet_dreams](https://github.com/user-attachments/assets/d34f8454-a3e0-4142-b636-547188c1ad2e)


## Assets

Planet textures by [Solar System Scope](https://www.solarsystemscope.com/textures/), licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).


