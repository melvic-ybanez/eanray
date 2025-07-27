local final_scene = require("examples.rt1w.final_scene_utils")
local final_scene_with_bouncing_spheres = require("examples.rtnw.final_scene_with_bouncing_spheres")

local objects = final_scene_with_bouncing_spheres.build_objects()

local world = engine.BVH:new(objects)
objects = engine.ObjectList:new()
objects:add(world)

return engine.Scene:new(final_scene.setup_camera(true), objects)