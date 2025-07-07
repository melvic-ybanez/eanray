local Sphere = engine.shapes.Sphere
local Point = engine.math.Point
local Lambertian = engine.materials.Lambertian
local Dielectric = engine.materials.Dielectric
local RefractiveIndex = Dielectric.RefractiveIndex
local Metal = engine.materials.Metal
local Color = engine.Color

local camera = engine.Camera:new(400, 16 / 9)
camera.samples_per_pixel = 100
camera.antialising = true
camera.max_depth = 50

local objects = engine.ObjectList:new()

objects:add_all(
-- ground
    Sphere:stationary(Point:new(0, -100.5, -1), 100, Lambertian:from_albedo(Color:new(0.8, 0.8, 0))),
-- center (slightly further)
    Sphere:stationary(Point:new(0, 0, -1.2), 0.5, Lambertian:from_albedo(Color:new(0.1, 0.2, 0.5))),
-- left
    Sphere:stationary(Point:new(-1, 0, -1), 0.5, Dielectric:new(RefractiveIndex.VACUUM / RefractiveIndex.WATER)),
-- right
    Sphere:stationary(Point:new(1, 0, -1), 0.5, Metal:new(Color:new(0.8, 0.6, 0.2), 1))
)

return engine.Scene:new(camera, objects)