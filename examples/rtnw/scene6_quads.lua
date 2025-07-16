local Color = engine.Color
local Lambertian = engine.materials.Lambertian
local Quad = engine.shapes.Quad
local Point = engine.math.Point
local Vec = engine.math.Vec

local left_red = Lambertian:from_albedo(Color:new(1.0, 0.2, 0.2))
local back_green = Lambertian:from_albedo(Color:new(0.2, 1.0, 0.2))
local right_blue = Lambertian:from_albedo(Color:new(0.2, 0.2, 1.0))
local upper_orange = Lambertian:from_albedo(Color:new(1.0, 0.5, 0.0))
local lower_teal = Lambertian:from_albedo(Color:new(0.2, 0.8, 0.8))

local objects = engine.ObjectList:new()

objects:add_all(
    Quad:new(Point:new(-3, -2, 5), Vec:new(0, 0, -4), Vec:new(0, 4, 0), left_red),
    Quad:new(Point:new(-2, -2, 0), Vec:new(4, 0, 0), Vec:new(0, 4, 0), back_green),
    Quad:new(Point:new(3, -2, 1), Vec:new(0, 0, 4), Vec:new(0, 4, 0), right_blue),
    Quad:new(Point:new(-2, 3, 1), Vec:new(4, 0, 0), Vec:new(0, 0, 4), upper_orange),
    Quad:new(Point:new(-2, -3, 5), Vec:new(4, 0, 0), Vec:new(0, 0, -4), lower_teal)
)

local cam = engine.Camera:new(400, 1.0)

cam.samples_per_pixel = 100
cam.max_depth         = 50

cam.field_of_view     = 80
cam.look_from = Point:new(0,0,9)
cam.look_at   = Point:new(0,0,0)
cam.vup      = Vec:new(0,1,0)

cam.defocus_angle = 0

return engine.Scene:new(cam, objects)