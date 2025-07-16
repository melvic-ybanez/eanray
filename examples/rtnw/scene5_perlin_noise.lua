local Point = engine.math.Point
local Lambertian = engine.materials.Lambertian
local Sphere = engine.shapes.Sphere

local pertext = engine.textures.Noise:new(4)
local objects = engine.ObjectList:new()

objects:add(Sphere:stationary(Point:new(0, -1000, 0), 1000, Lambertian:new(pertext)))
objects:add(Sphere:stationary(Point:new(0, 2, 0), 2, Lambertian:new(pertext)))

local cam = engine.Camera:new(400, 16 / 9)

cam.samples_per_pixel = 100
cam.max_depth         = 50

cam.field_of_view     = 20
cam.look_from = Point:new(13, 2, 3)
cam.look_at   = Point.ZERO
cam.vup      = engine.math.Vec:new(0, 1, 0)

cam.defocus_angle = 0

return engine.Scene:new(cam, objects)