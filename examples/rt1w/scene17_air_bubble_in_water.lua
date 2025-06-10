local Sphere = engine.shapes.Sphere
local Point = engine.math.Point
local Lambertian = engine.materials.Lambertian
local Dielectric = engine.materials.Dielectric
local RefractiveIndex = Dielectric.RefractiveIndex
local Metal = engine.materials.Metal
local Color = engine.Color

local camera = {
  aspect_ratio = 16 / 9,
  image_width = 400,
  samples_per_pixel = 100,
  antialising = true,
  max_depth = 50,
}

return {
  camera = camera,
  objects = {
    Sphere:new(Point:new(0, -100.5, -1), 100, Lambertian:new(Color:new(0.8, 0.8, 0))),
    ---- center (slightly further)
    Sphere:new(Point:new(0, 0, -1.2), 0.5, Lambertian:new(Color:new(0.1, 0.2, 0.5))),
    ---- left
    Sphere:new(Point:new(-1, 0, -1), 0.5, Dielectric:new(RefractiveIndex.VACUUM / RefractiveIndex.WATER)),
    ---- right
    Sphere:new(Point:new(1, 0, -1), 0.5, Metal:new(Color:new(0.8, 0.6, 0.2), 1))
  }
}