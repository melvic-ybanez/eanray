local final_scene = require("examples.rt1w.final_scene_utils")
local final_scene_with_bouncing_spheres = require("examples.rtnw.final_scene_with_bouncing_spheres")

local Color = engine.Color
local Point = engine.math.Point
local Lambertian = engine.materials.Lambertian
local Sphere = engine.shapes.Sphere
local ObjectList = engine.ObjectList

local objects = ObjectList:new()

local function make_ground()
  local radius = 1000
  local checker = engine.textures.Checker:from_colors(0.32, Color:new(.2, .3, .1), Color:new(.9, .9, .9))
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:from_texture(checker))
  objects:add(ground)
end

final_scene_with_bouncing_spheres.build_random_spheres(objects)

make_ground()
final_scene.make_big_3_spheres(objects)

local world = engine.BVH:new(objects)
objects = ObjectList:new()
objects:add(world)

return engine.Scene:new(final_scene.setup_camera(false), objects)