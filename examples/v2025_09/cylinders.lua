local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Lambertian = engine.materials.Lambertian
local Sphere = engine.shapes.Sphere
local ObjectList = engine.ObjectList
local DiffuseLight = engine.materials.DiffuseLight
local Cylinder = engine.shapes.Cylinder

local objects = ObjectList:new()

local function make_ground()
  local radius = 1000
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:from_albedo(Color:new(0.5, 0.5, 0.5)))
  objects:add(ground)
end

local function make_cylinder_group()
  for i = 0, 6 do
    local height = 0.25 + 0.2 * i
    local radius = 1.25 * 0.65 ^ i
    local side_mat = Lambertian:from_albedo(Color:new(1, 1,1))
    local cylinder

    if i == 0 then
      cylinder = Cylinder:open(radius, height, side_mat)
    else
      cylinder = Cylinder:closed(radius, height, side_mat, side_mat)
    end

    objects:add(cylinder:translate(4, height / 2, 0))
  end
end

local function make_light()
  local outer_light_radius = 7
  local outer_light = DiffuseLight:from_emission(Color:new(1, 1, 1))
  objects:add(Sphere:stationary(Point:new(4, outer_light_radius + 3, 2.3), outer_light_radius, outer_light))
end

local function make_moon()
  local mat = engine.textures.Image:new("examples/images/moon.jpg")
  local glass_sphere = Sphere:stationary(Point:new(-1000, 30, 2.5), 40, DiffuseLight:from_texture(mat))
  objects:add(glass_sphere)
end

local function make_capped_cylinder(x, z)
  local height = 0.15
  local radius = 0.8
  local side_mat = Lambertian:from_albedo(Color:new(1, 1, 1))
  local cylinder = Cylinder:closed(radius, height, side_mat, side_mat)
  local translate = cylinder:translate(x, height / 2, z)
  objects:add(translate)
end

local function setup_bvh()
  local world = engine.BVH:new(objects)
  objects = ObjectList:new()
  objects:add(world)
end

local function setup_camera()
  local camera = engine.Camera:new(1200, 16 / 9)
  camera.samples_per_pixel = 500
  camera.max_depth = 50

  camera.field_of_view = 20
  camera.look_from = Point:new(13, 2, 3)
  camera.look_at = Point.ZERO
  camera.vup = Vec:new(0, 1, 0)

  camera.defocus_angle = 0.6
  camera.focus_distance = 10
  return camera
end

make_light()
make_ground()
make_cylinder_group()
make_capped_cylinder(1.5, 3)
make_capped_cylinder(-1, -4)
make_capped_cylinder(6, 3.5)
make_capped_cylinder(5.5, -2)
make_capped_cylinder(-5, 1)
make_moon()
setup_bvh()

return engine.Scene:new(setup_camera(), objects)