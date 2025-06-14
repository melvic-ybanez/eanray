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
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:new(Color:new(0.76, 0.70, 0.50)))
  objects:add(ground)
end

make_ground()

objects:add(Sphere:stationary(Point:new(-4, 1, 0), 1.0, Lambertian:new(Color:new(0.4, 0.2, 0.1))))
objects:add(Sphere:stationary(Point:new(4, 1, 0), 1.0, Metal:new(Color:new(0.7, 0.6, 0.5), 0)))

local function make_group()
  local base_radius = 0.6
  local head_radius = 0.4
  local z = 2.5
  objects:add(Sphere:stationary(Point:new(3, base_radius, z), base_radius, Metal:new(Color:new(1, 0.5, 0.5), 0)))
  objects:add(Sphere:stationary(Point:new(3, base_radius * 2 + head_radius, z), head_radius, Dielectric:new(Dielectric.RefractiveIndex.GLASS)))
end

make_group()

local small_radius = 0.25

local function make_small_lambertian(center)
  local albedo = Color.random() * Color.random()
  objects:add(Sphere:stationary(center, small_radius, Lambertian:new(albedo)))
end

local function make_small_metal(center)
  local albedo = Color.random(0.5, 1)
  local fuzz = engine.math.random_range(0, 0.5)
  objects:add(Sphere:stationary(center, small_radius, Metal:new(albedo, fuzz)))
end

local function make_small_glass(center)
  local sphere_material = Dielectric:new(Dielectric.RefractiveIndex.GLASS)
  objects:add(Sphere:stationary(center, small_radius, sphere_material))
end

make_small_lambertian(Point:new(5.5, small_radius, 0))
make_small_lambertian(Point:new(2.5, small_radius, 1))
make_small_metal(Point:new(5, small_radius, 2))
make_small_glass(Point:new(4, small_radius, 3.3))
make_small_lambertian(Point:new(2, small_radius, -2.5))
make_small_lambertian(Point:new(-3.8, small_radius, 3.3))
make_small_lambertian(Point:new(-3.5, small_radius, 2.5))
make_small_metal(Point:new(-4.1, small_radius, 1.4))
objects:add(Sphere:stationary(Point:new(5.6, 0.2, 2.7), 0.2, Lambertian:new(Color.random() * Color.random())))

objects:add(Sphere:stationary(Point:new(5.7, small_radius, 1), small_radius, Dielectric:new(Dielectric.RefractiveIndex.GLASS)))

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