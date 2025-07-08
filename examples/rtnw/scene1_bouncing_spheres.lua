local final_scene = require("examples.rt1w.final_scene_utils")
local final_scene_with_bouncing_spheres = require("examples.rtnw.final_scene_with_bouncing_spheres")

return engine.Scene:new(final_scene.setup_camera(false), final_scene_with_bouncing_spheres.build_objects())