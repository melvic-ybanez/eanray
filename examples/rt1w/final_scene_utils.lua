local common = require("examples.rt1w.common")

local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Lambertian = engine.materials.Lambertian
local Metal = engine.materials.Metal
local Dielectric = engine.materials.Dielectric
local Sphere = engine.shapes.Sphere

local function make_ground(objects)
  local radius = 1000
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:from_albedo(Color:new(0.5, 0.5, 0.5)))
  objects:add(ground)
end

local function make_big_3_spheres(objects)
  objects:add_all(
      Sphere:stationary(Point:new(0, 1, 0), 1.0, Dielectric:new(Dielectric.RefractiveIndex.GLASS)),
      Sphere:stationary(Point:new(-4, 1, 0), 1.0, Lambertian:from_albedo(Color:new(0.4, 0.2, 0.1))),
      Sphere:stationary(Point:new(4, 1, 0), 1.0, Metal:new(Color:new(0.7, 0.6, 0.5), 0))
  )
end

local function setup_camera(is_high)
  local camera = engine.Camera:new(is_high and 1200 or 400, 16 / 9)
  camera.samples_per_pixel = is_high and 500 or 100
  camera.max_depth = 50

  camera.field_of_view = 20
  camera.look_from = Point:new(13, 2, 3)
  camera.look_at = Point.ZERO
  camera.vup = Vec:new(0, 1, 0)

  camera.defocus_angle = 0.6
  camera.focus_distance = 10
  camera.background = common.background

  return camera
end

return {
  make_ground = make_ground,
  make_big_3_spheres = make_big_3_spheres,
  setup_camera = setup_camera
}