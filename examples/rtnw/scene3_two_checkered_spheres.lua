local final_scene = require("examples.rt1w.final_scene_utils")

local Sphere = engine.shapes.Sphere
local Point = engine.math.Point
local Color = engine.Color
local Lambertian = engine.materials.Lambertian

local checker = engine.textures.Checker:from_colors(
    0.32,
    Color:new(.2, .3, .1),
    Color:new(0.9, 0.9, 0.9)
)

local objects = engine.ObjectList:new()

objects:add(Sphere:stationary(Point:new(0, -10, 0), 10, Lambertian:new(checker)))
objects:add(Sphere:stationary(Point:new(0, 10, 0), 10, Lambertian:new(checker)))

local camera = final_scene.setup_camera(false)
camera.defocus_angle = 0

return engine.Scene:new(camera, objects)