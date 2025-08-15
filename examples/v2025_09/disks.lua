local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Disk = engine.shapes.Disk
local DiffuseLight = engine.materials.DiffuseLight

local red = DiffuseLight:from_emission(Color:new(1.0, 0.2, 0.2))
local green = DiffuseLight:from_emission(Color:new(0.2, 1.0, 0.2))
local blue = DiffuseLight:from_emission(Color:new(0.2, 0.2, 1.0))

local objects = engine.ObjectList:new()

objects:add_all(
    Disk:new(Point:new(-1.6, 0, 0.5), Vec:new(0.9, -0.9, 0), Vec:new(0, 1, 0), 1, red),
    Disk:new(Point:new(0, 0, 0.5), Vec:new(0.75, 0, 0), Vec:new(0, 1, 0), 1, green),
    Disk:new(Point:new(1.6, 0, 0.5), Vec:new(0.9, 0.9, 0), Vec:new(0, 1, 0), 1, blue)
)

local cam = engine.Camera:new(400, 1.0)

cam.samples_per_pixel = 100
cam.max_depth = 50

cam.field_of_view = 80
cam.look_from = Point:new(0, 0, 4.5)
cam.look_at = Point:new(0, 0, 0)
cam.vup = Vec:new(0, 1, 0)

cam.defocus_angle = 0

return engine.Scene:new(cam, objects)