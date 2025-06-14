local Sphere = engine.shapes.Sphere
local Lambertian = engine.materials.Lambertian
local Dielectric = engine.materials.Dielectric
local Metal = engine.materials.Metal
local Point = engine.math.Point
local Color = engine.Color

local camera = engine.Camera:new(400, 16 / 9)
camera.samples_per_pixel = 100
camera.antialising = true
camera.max_depth = 50

local objects = engine.ObjectList:new();

-- ground
objects:add(Sphere:stationary(Point:new(0, -100.5, -1), 100, Lambertian:new(Color:new(0.8, 0.8, 0))))

-- center (slightly further)
objects:add(Sphere:stationary(Point:new(0, 0, -1.2), 0.5, Lambertian:new(Color:new(0.1, 0.2, 0.5))))

-- left
objects:add(Sphere:stationary(Point:new(-1, 0, -1), 0.5, Dielectric:new(Dielectric.RefractiveIndex.GLASS)))

-- right
objects:add(Sphere:stationary(Point:new(1, 0, -1), 0.5, Metal:new(Color:new(0.8, 0.6, 0.2), 1)))

return engine.Scene:new(camera, objects)