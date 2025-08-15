local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec

local green = engine.materials.DiffuseLight:from_emission(Color:new(0.2, 1.0, 0.2))

local objects = engine.ObjectList:new()

objects:add(engine.shapes.Triangle:new(Point:new(-2, -2, 0), Vec:new(4, 0, 0), Vec:new(2, 4, 0), green))

local cam = engine.Camera:new(400, 1.0)

cam.samples_per_pixel = 100
cam.max_depth = 50

cam.field_of_view = 80
cam.look_from = Point:new(0, 0, 4.5)
cam.look_at = Point:new(0, 0, 0)
cam.vup = Vec:new(0, 1, 0)

cam.defocus_angle = 0

return engine.Scene:new(cam, objects)