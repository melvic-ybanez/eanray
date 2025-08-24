local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Lambertian = engine.materials.Lambertian
local Metal = engine.materials.Metal
local Sphere = engine.shapes.Sphere
local Cylinder = engine.shapes.Cylinder
local Cone = engine.shapes.Cone
local Quad = engine.shapes.Quad
local ObjectList = engine.ObjectList
local textures = engine.textures
local Image = textures.Image
local DiffuseLight = engine.materials.DiffuseLight
local Translate = engine.transforms.Translate
local RotateX = engine.transforms.RotateX
local RotateY = engine.transforms.RotateY

local objects = ObjectList:new()

local function make_ground()
  local radius = 1000
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:from_albedo(Color:from_scalar(0.5)))
  objects:add(ground)
end

local wood_radius = 0.1

local function make_sun()
  local radius = 0.6

  -- higher red, moderate green and lower blue, to make it warm
  local intensity = Color:new(3.5, 2.9, 2.2)

  local sun = DiffuseLight:from_texture_intensified(Image:new("examples/images/sun.jpg"), intensity)
  objects:add(Sphere:stationary(Point:new(3.5, radius + wood_radius * 2, 2), radius, sun))
end

local function make_piles_of_wood()
  local x = 3
  local left = 2.5
  local bark_mat = Lambertian:from_albedo(Color:new(0.235, 0.157, 0.098))  -- dark earthy brown
  local cut_wood_mat = Lambertian:from_albedo(Color:new(0.784, 0.667, 0.471))  -- pale golden wood
  local wood = Cylinder:closed(wood_radius, 1.4, bark_mat, cut_wood_mat)

  local rotated_cw = RotateY:new(RotateX:new(wood, 90), -45)
  local rotated_ccw = RotateY:new(RotateX:new(wood, 90), 45)

  local bottom_left = Translate:new(rotated_cw, Vec:new(x + 0.7, wood_radius, left - 0.2))
  local bottom_right = Translate:new(rotated_cw, Vec:new(x + 0.3, wood_radius, left - 0.7))

  local middle_left = Translate:new(rotated_ccw, Vec:new(x, wood_radius * 3, left - 0.15))
  local middle_right = Translate:new(rotated_ccw, Vec:new(x + 0.8, wood_radius * 3, left - 0.9))

  local upper_left = Translate:new(rotated_cw, Vec:new(x + 1, wood_radius * 5, left - 0.2))
  local upper_right = Translate:new(rotated_cw, Vec:new(x + 0.2, wood_radius * 5, left - 0.7))

  objects:add_all(bottom_left, bottom_right, middle_left, middle_right, upper_left, upper_right)
end

local function make_table()
  local x = 4
  local z = -0.7
  local legs_height = 0.7
  local top_height = 0.09
  local surface_y = top_height + legs_height
  local table_mat = Lambertian:from_albedo(Color.WHITE)

  local top = Translate:new(Cylinder:closed(1.25, top_height, table_mat, table_mat), Vec:new(x, surface_y - top_height / 2, z))

  local legs_half_distance = 0.6
  local leg_base_radius = 0.06
  local leg = Cone:frustum_closed(leg_base_radius, leg_base_radius * 0.5, legs_height, table_mat, table_mat)

  local legs_y = 0
  local back_left_leg = Translate:new(leg, Vec:new(x - legs_half_distance, legs_y, z + legs_half_distance))
  local back_right_leg = Translate:new(leg, Vec:new(x - legs_half_distance, legs_y, z - legs_half_distance))
  local front_left_leg = Translate:new(leg, Vec:new(x + legs_half_distance, legs_y, z + legs_half_distance))
  local front_right_leg = Translate:new(leg, Vec:new(x + legs_half_distance, legs_y, z - legs_half_distance))

  local map_size = 1
  local half_map_size = map_size / 2
  local map_mat = Lambertian:new(Image:new("examples/images/planets/earth.jpg"))
  local map = Quad:new(Point:new(x - half_map_size, surface_y, z + half_map_size), Vec:new(map_size, 0, 0), Vec:new(0, 0, -map_size), map_mat)

  local jupiter_radius = 0.35
  local jupiter = Sphere:new(
      Point:new(x - half_map_size, surface_y + jupiter_radius, z - half_map_size), jupiter_radius,
      Lambertian:new(Image:new("examples/images/planets/jupiter.jpg")))

  local saturn_radius = jupiter_radius * 0.4
  local saturn = Sphere:new(
      Point:new(x - half_map_size, surface_y + saturn_radius, z + half_map_size), saturn_radius,
      Lambertian:new(Image:new("examples/images/planets/saturn.jpg")))

  local meta_sphere_radius = jupiter_radius * 0.5
  local metal_sphere = Sphere:new(
      Point:new(x + half_map_size, surface_y + meta_sphere_radius, z - half_map_size - 0.05), meta_sphere_radius,
      Metal:new(Color.WHITE, 0))

  local lamp_cover_base_radius = 0.35
  local lamp_cover_apex_radius = lamp_cover_base_radius * 0.4
  local lamp_cover_height = 0.55
  local lamp_x = x
  local lamp_cover_mat = Lambertian:from_albedo(Color.WHITE)
  local bulb_radius = 0.22
  local lamp_cover = Translate:new(
      RotateX:new(Cone:frustum_open(lamp_cover_base_radius, lamp_cover_apex_radius, lamp_cover_height, lamp_cover_mat), 135),
      Vec:new(lamp_x, surface_y + 1.1 + bulb_radius, z - map_size - bulb_radius - 0.1))
  local lamp_bulb_mat = DiffuseLight:from_emission_intensified(Color:new(1.0, 0.84, 0.66), Color:from_scalar(2))
  local lamp_bulb = Sphere:new(Point:new(lamp_x, surface_y + 1.11, z - map_size - 0.11), bulb_radius, lamp_bulb_mat)

  objects:add_all(top,
      back_left_leg, back_right_leg, front_left_leg, front_right_leg, map,
      jupiter, saturn, lamp_cover, metal_sphere, lamp_bulb)
end

local function make_map_on_ground()
  local map_size = 1
  local map_mat = Lambertian:new(Image:new("examples/images/planets/venus_surface.jpg"))
  local map = Quad:new(Point:new(4.5 - map_size / 2, 0, 1.5), Vec:new(map_size, 0, 0), Vec:new(0, 0, -map_size), map_mat)
  objects:add(map)
end

local function make_stones()
  local radius = 5.5
  local mat = Lambertian:from_albedo(Color.WHITE)
  local right = Sphere:new(Point:new(-2.5, radius, -4.5), radius, mat)

  local left_height = 5
  local left = Translate:new(Cylinder:closed(4, left_height, mat, mat), Vec:new(-2, left_height / 2, 5))

  objects:add_all(right, left)
end

local function setup_camera()
  local camera = engine.Camera:new(1200, 16 / 9)
  camera.samples_per_pixel = 5000
  camera.max_depth = 50

  camera.field_of_view = 20
  camera.look_from = Point:new(12.5, 2.5, 2.3)
  camera.look_at = Point.ZERO
  camera.vup = Vec:new(0, 1, 0)

  camera.defocus_angle = 0.6
  camera.focus_distance = 10

  return camera
end

local function setup_bvh()
  local world = engine.BVH:new(objects)
  objects = ObjectList:new()
  objects:add(world)
end

make_ground()
make_sun()
make_piles_of_wood()
make_table()
make_map_on_ground()
make_stones()

setup_bvh()

return engine.Scene:new(setup_camera(), objects)