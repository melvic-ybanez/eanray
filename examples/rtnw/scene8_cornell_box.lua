local Lambertian = engine.materials.Lambertian
local Quad = engine.shapes.Quad
local Box = engine.shapes.Box
local Translate = engine.transforms.Translate
local RotateY = engine.transforms.RotateY
local Point = engine.math.Point
local Color = engine.Color
local Vec = engine.math.Vec

local red = Lambertian:from_albedo(Color:new(.65, 0.05, 0.05))
local white = Lambertian:from_albedo(Color:new(0.73, 0.73, 0.73))
local green = Lambertian:from_albedo(Color:new(0.12, 0.45, 0.15))
local light = engine.materials.DiffuseLight:from_emission(Color:new(15, 15, 15))

local objects = engine.ObjectList:new()

objects:add_all(
    Quad:new(Point:new(555, 0, 0), Vec:new(0, 555, 0), Vec:new(0, 0, 555), green),
    Quad:new(Point:new(0, 0, 0), Vec:new(0, 555, 0), Vec:new(0, 0, 555), red),
    Quad:new(Point:new(343, 554, 332), Vec:new(-130, 0, 0), Vec:new(0, 0, -105), light),
    Quad:new(Point:new(0, 0, 0), Vec:new(555, 0, 0), Vec:new(0, 0, 555), white),
    Quad:new(Point:new(555, 555, 555), Vec:new(-555, 0, 0), Vec:new(0, 0, -555), white),
    Quad:new(Point:new(0, 0, 555), Vec:new(555, 0, 0), Vec:new(0, 555, 0), white)
)

local box1 = Box:new(Point:new(0, 0, 0), Point:new(165, 330, 165), white)
box1 = RotateY:new(box1, 15)
box1 = Translate:new(box1, Vec:new(265, 0, 295))
objects:add(box1)

local box2 = Box:new(Point:new(0, 0, 0), Point:new(165, 165, 165), white)
box2 = RotateY:new(box2, -18)
box2 = Translate:new(box2, Vec:new(130, 0, 65))
objects:add(box2)

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