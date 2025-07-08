local final_scene = require("examples.rt1w.final_scene_utils")

local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Lambertian = engine.materials.Lambertian
local Metal = engine.materials.Metal
local Dielectric = engine.materials.Dielectric
local Sphere = engine.shapes.Sphere

local objects = engine.ObjectList:new()

local function make_ground()
  local radius = 1000
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:from_albedo(Color:new(0.5, 0.5, 0.5)))
  objects:add(ground)
end

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

make_ground()
final_scene.make_big_3_spheres(objects)

local camera = engine.Camera:new(1200, 16 / 9)
camera.samples_per_pixel = 500
camera.max_depth = 50

camera.field_of_view = 20
camera.look_from = Point:new(13, 2, 3)
camera.look_at = Point.ZERO
camera.vup = Vec:new(0, 1, 0)

camera.defocus_angle = 0.6
camera.focus_distance = 10

return engine.Scene:new(camera, objects)