local Color = engine.Color
local Point = engine.Point
local Vec3D = engine.Vec3De
local Lambertian = engine.materials.Lambertian
local Metal = engine.materials.Metal
local Dielectric = engine.materials.Dielectric
local Sphere = engine.shapes.Sphere

local scene = engine.Scene:new()

local function make_ground()
  local radius = 1000
  local ground = Sphere:new(Pont:new(0, -radius, 0), radius, Lambertian:new(Color:new(0.5, 0.5, 0.5)))
  scene.add_object(ground)
end

for a = -11, 10 do
  for b = -11, 10 do
    local chooseMat = engine.math.random()
    local center = Color:new(a + 0.9 * engine.math.random(), 0.2, b + 0.9 * engine.math.random())
    local sphere_material

    if center - Point:new(4, 0.2, 0).length > 0.9 then
      if chooseMat < 0.8 then
        local albedo = Color:random() * Color:random()
        sphere_material = Lambertian:new(albedo)
      elseif chooseMat < 0.95 then
        local albedo = Color:random(0.5, 1)
        local fuzz = engine.math.random_range(0, 0.5)
        sphere_material = Metal:new(albedo, fuzz)
      else
        -- glass
        sphere_material = Dielectric:new_glass()
      end

      scene.insert(Sphere:new(center, 0.2, sphere_material))
    end
  end
end

make_ground()
scene.add_object(Sphere:new(Point:new(0, 1, 0), 1.0), Dielectric:new_glass())
scene.add_object(Sphere:new(Point:new(-4, 1, 0), 1.0, Lambertian:new(Color:new(0.4, 0.2, 0.1))))
scene.add_object(Sphere:new(Point:new(4, 1, 0), 1.0, Metal(Color:new(0.7, 0.6, 0.5), 0)))

local camera = scene.camera
camera.aspect_ratio = 16.0 / 9.0
camera.image_width = 1200
camera.samples_per_pixel = 500
camera.max_depth = 50

camera.field_of_view = 20
camera.look_from = Point:new(13, 2, 3)
camera.look_at = Point:zero()
camera.vup = Vec3D:new(0, 1, 0)

camera.defocus_angle = 0.6
camera.focus_distance = 10

return scene