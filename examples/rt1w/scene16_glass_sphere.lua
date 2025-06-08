local Sphere = engine.shapes.Sphere
local Lambertian = engine.materials.Lambertian
local Dielectric = engine.materials.Dielectric
local Metal = engine.materials.Metal
local Point = engine.Point
local Color = engine.Color

local camera = {}
camera.aspect_ratio = 16 / 9
camera.image_width = 400
camera.samples_per_pixel = 100
camera.antialising = true
camera.max_depth = 50

return {
  camera = camera,
  objects = {
    -- ground
    Sphere:new(Point:new(0, -100.5, -1), 100, Lambertian:new(Color:new(0.8, 0.8, 0))),
    ---- center (slightly further)
    Sphere:new(Point:new(0, 0, -1.2), 0.5, Lambertian:new(Color:new(0.1, 0.2, 0.5))),
    ---- left
    Sphere:new(Point:new(-1, 0, -1), 0.5, Dielectric:new_glass()),
    ---- right
    Sphere:new(Point:new(1, 0, -1), 0.5, Metal:new(Color:new(0.8, 0.6, 0.2), 1))
  }
}