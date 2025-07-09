local final_scene = require("examples.rt1w.final_scene_utils")

local Color = engine.Color
local Point = engine.math.Point
local Lambertian = engine.materials.Lambertian
local Metal = engine.materials.Metal
local Dielectric = engine.materials.Dielectric
local Sphere = engine.shapes.Sphere

local objects = engine.ObjectList:new()

for a = -11, 10 do
  for b = -11, 10 do
    local chooseMat = engine.math.random()
    local center = Point:new(a + 0.9 * engine.math.random(), 0.2, b + 0.9 * engine.math.random())
    local sphere_material

    if (center - Point:new(4, 0.2, 0)):length() > 0.9 then
      if chooseMat < 0.8 then
        local albedo = Color.random() * Color.random()
        sphere_material = Lambertian:from_albedo(albedo)
      elseif chooseMat < 0.95 then
        local albedo = Color.random_range(0.5, 1)
        local fuzz = engine.math.random_range(0, 0.5)
        sphere_material = Metal:new(albedo, fuzz)
      else
        sphere_material = Dielectric:new(Dielectric.RefractiveIndex.GLASS)
      end

      objects:add(Sphere:stationary(center, 0.2, sphere_material))
    end
  end
end

final_scene.make_ground(objects)
final_scene.make_big_3_spheres(objects)

return engine.Scene:new(final_scene.setup_camera(true), objects)