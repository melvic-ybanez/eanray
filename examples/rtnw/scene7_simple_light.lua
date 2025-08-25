local common = require("examples.rt1w.common")

local Point = engine.math.Point
local Lambertian = engine.materials.Lambertian
local Color = engine.Color
local Vec = engine.math.Vec
local Sphere = engine.shapes.Sphere

local pertext = engine.textures.Noise:new(4, Color:new(0.5, 0.5, 0.5))
local objects = engine.ObjectList:new()

objects:add(Sphere:stationary(Point:new(0, -1000, 0), 1000, Lambertian:from_texture(pertext)))
objects:add(Sphere:stationary(Point:new(0, 2, 0), 2, Lambertian:from_texture(pertext)))

local diff_light = engine.materials.DiffuseLight:from_emission(Color:new(4, 4, 4))
objects:add(engine.shapes.Quad:new(Point:new(3, 1, -2), Vec:new(2, 0, 0), Vec:new(0, 2, 0), diff_light))

local cam = engine.Camera:new(400, 16 / 9)
cam.samples_per_pixel = 100
cam.max_depth = 50
cam.background = Color.ZERO

cam.field_of_view = 20
cam.look_from = Point:new(26, 3, 6)
cam.look_at = Point:new(0, 2, 0)
cam.vup = Vec:new(0, 1, 0)

cam.defocus_angle = 0

cam.background = common.background

return engine.Scene:new(cam, objects)