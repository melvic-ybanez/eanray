local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Lambertian = engine.materials.Lambertian
local Dielectric = engine.materials.Dielectric
local Sphere = engine.shapes.Sphere
local ObjectList = engine.ObjectList
local DiffuseLight = engine.materials.DiffuseLight
local Cylinder = engine.shapes.Cylinder
local Translate = engine.transforms.Translate

local objects = ObjectList:new()

local function make_ground()
  local radius = 1000
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:from_albedo(Color:new(0.5, 0.5, 0.5)))
  objects:add(ground)
end

make_ground()

for i = 0, 6 do
  local height = 0.25 + 0.2 * i
  local radius = 1.25 * 0.65 ^ i
  local cylinder = Cylinder:finite(radius, height, Lambertian:from_albedo(Color:new(1, 1,1)))
  local translate = Translate:new(cylinder, Vec:new(4, height / 2, 0))
  objects:add(translate)
end

local outer_light_radius = 7
local outer_light = DiffuseLight:from_emission(Color:new(1, 1, 1))
objects:add(Sphere:stationary(Point:new(4, outer_light_radius + 3, 2.3), outer_light_radius, outer_light))

local mat = engine.textures.Image:new("examples/images/moon.jpg")
local glass_sphere = Sphere:stationary(Point:new(-1000, 30, 2.5), 40, DiffuseLight:from_texture(mat))
objects:add(glass_sphere)

local camera = engine.Camera:new(1200, 16 / 9)
camera.samples_per_pixel = 500
camera.max_depth = 50

camera.field_of_view = 20
camera.look_from = Point:new(13, 2, 3)
camera.look_at = Point.ZERO
camera.vup = Vec:new(0, 1, 0)

camera.defocus_angle = 0.6
camera.focus_distance = 10

local world = engine.BVH:new(objects)
objects = ObjectList:new()
objects:add(world)

return engine.Scene:new(camera, objects)