local final_scene = require("examples.rt1w.final_scene_utils")

local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Lambertian = engine.materials.Lambertian
local Metal = engine.materials.Metal
local Dielectric = engine.materials.Dielectric
local Sphere = engine.shapes.Sphere

local function build_random_spheres(objects)
  for a = -11, 10 do
    for b = -11, 10 do
      local chooseMat = engine.math.random()
      local center = Point:new(a + 0.9 * engine.math.random(), 0.2, b + 0.9 * engine.math.random())
      local material

      if (center - Point:new(4, 0.2, 0)):length() > 0.9 then
        if chooseMat < 0.8 then
          local albedo = Color.random() * Color.random()
          material = Lambertian:from_albedo(albedo)
          local center2 = center + Vec:new(0, engine.math.random_range(0, 0.5), 0)
          objects:add(Sphere:moving(center, center2, 0.2, material))
        elseif chooseMat < 0.95 then
          local albedo = Color.random_range(0.5, 1)
          local fuzz = engine.math.random_range(0, 0.5)
          material = Metal:new(albedo, fuzz)
          objects:add(Sphere:stationary(center, 0.2, material))
        else
          material = Dielectric:new(Dielectric.RefractiveIndex.GLASS)
          objects:add(Sphere:stationary(center, 0.2, material))
        end
      end
    end
  end
end

local function build_objects()
  local objects = engine.ObjectList:new()

  build_random_spheres(objects)
  final_scene.make_ground(objects)
  final_scene.make_big_3_spheres(objects)

  return objects
end

return {
  build_objects = build_objects,
  build_random_spheres = build_random_spheres
}