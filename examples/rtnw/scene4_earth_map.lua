local common = require("examples.rt1w.common")

local Point = engine.math.Point

local earth_texture = engine.textures.Image:new("examples/images/earthmap.jpg")
local earth_surface = engine.materials.Lambertian:new(earth_texture)
local globe = engine.shapes.Sphere:stationary(Point:new(0, 0, 0), 2, earth_surface)

local cam = engine.Camera:new(400, 16 / 9)
cam.samples_per_pixel = 100
cam.max_depth = 50

cam.field_of_view = 20
cam.look_from = Point:new(0, 0, 12)
cam.look_at = Point.ZERO
cam.vup = engine.math.Vec:new(0, 1, 0)

cam.defocus_angle = 0

cam.background = common.background

local objects = engine.ObjectList:new()
objects:add(globe)

return engine.Scene:new(cam, objects)