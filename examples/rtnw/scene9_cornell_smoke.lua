local Point = engine.math.Point
local Color = engine.Color
local Vec = engine.math.Vec
local Box = engine.shapes.Box
local Translate = engine.transforms.Translate
local RotateY = engine.transforms.RotateY
local Quad = engine.shapes.Quad
local Lambertian = engine.materials.Lambertian
local ConstantMedium = engine.shapes.ConstantMedium

local red = Lambertian:from_albedo(Color:new(.65, 0.05, 0.05))
local white = Lambertian:from_albedo(Color:new(0.73, 0.73, 0.73))
local green = Lambertian:from_albedo(Color:new(0.12, 0.45, 0.15))
local light = engine.materials.DiffuseLight:from_emission(Color:new(7, 7, 7))

local objects = engine.ObjectList:new()

objects:add_all(
    Quad:new(Point:new(555, 0, 0), Vec:new(0, 555, 0), Vec:new(0, 0, 555), green),
    Quad:new(Point:new(0, 0, 0), Vec:new(0, 555, 0), Vec:new(0, 0, 555), red),
    Quad:new(Point:new(113, 554, 127), Vec:new(330, 0, 0), Vec:new(0, 0, 305), light),
    Quad:new(Point:new(0, 555, 0), Vec:new(555, 0, 0), Vec:new(0, 0, 555), white),
    Quad:new(Point:new(0, 0, 0), Vec:new(555, 0, 0), Vec:new(0, 0, 555), white),
    Quad:new(Point:new(0, 0, 555), Vec:new(555, 0, 0), Vec:new(0, 555, 0), white)
)

local box1 = Box:new(Point:new(0, 0, 0), Point:new(165, 330, 165), white)
    :transform(RotateY:new(15):and_then(Translate:new(265, 0, 295)))

local box2 = Box:new(Point:new(0, 0, 0), Point:new(165, 165, 165), white)
    :transform(RotateY:new(-18):and_then(Translate:new(130, 0, 65)))

objects:add_all(
    ConstantMedium:from_albedo(box1, 0.01, Color:new(0, 0, 0)),
    ConstantMedium:from_albedo(box2, 0.01, Color:new(1, 1, 1))
)

local cam = engine.Camera:new(600, 1)
cam.samples_per_pixel = 200
cam.max_depth = 50
cam.background = engine.Background:from_color(Color:new(0, 0, 0))

cam.field_of_view = 40
cam.look_from = Point:new(278, 278, -800)
cam.look_at = Point:new(278, 278, 0)
cam.vup = Vec:new(0, 1, 0)

cam.defocus_angle = 0

return engine.Scene:new(cam, objects)

