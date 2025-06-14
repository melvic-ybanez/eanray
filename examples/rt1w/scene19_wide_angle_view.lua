local Sphere = engine.shapes.Sphere
local Point = engine.math.Point
local Lambertian = engine.materials.Lambertian
local Color = engine.Color

local radius = math.cos(math.pi / 4)

local objects = engine.ObjectList:new()

objects:add_all(
    Sphere:stationary(Point:new(-radius, 0, -1), radius, Lambertian:new(Color:new(0, 0, 1))),
    Sphere:stationary(Point:new(radius, 0, -1), radius, Lambertian:new(Color:new(1, 0, 0)))
)

local camera = engine.Camera:new(400, 16 / 9)
camera.samples_per_pixel = 100
camera.max_depth = 50
camera.field_of_view = 90

return engine.Scene:new(camera, objects)