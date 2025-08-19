local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Lambertian = engine.materials.Lambertian
local Sphere = engine.shapes.Sphere
local Cylinder = engine.shapes.Cylinder
local Quad = engine.shapes.Quad
local Box = engine.shapes.Box
local ObjectList = engine.ObjectList
local textures = engine.textures
local Image = textures.Image
local DiffuseLight = engine.materials.DiffuseLight
local Metal = engine.materials.Metal
local Translate = engine.transforms.Translate
local RotateX = engine.transforms.RotateX
local RotateY = engine.transforms.RotateY

local objects = ObjectList:new()

local function make_ground()
  local radius = 1000
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:from_albedo(Color:new(0.5, 0.5, 0.5)))
  objects:add(ground)
end

local wood_radius = 0.1

local function make_sun()
  local radius = 0.6
  local sun = DiffuseLight:from_texture(Image:new("examples/images/sun.jpg"))
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
  local table_mat = Lambertian:from_albedo(Color:new(1, 1, 1))

  local top = Translate:new(Cylinder:closed(1.25, top_height, table_mat, table_mat), Vec:new(x, surface_y - top_height / 2, z))

  local legs_half_distance = 0.5
  local legs_radius = 0.06
  local leg = Cylinder:closed(legs_radius, legs_height, table_mat, table_mat)

  local back_left_leg = Translate:new(leg, Vec:new(x - legs_half_distance, legs_height / 2, z + legs_half_distance))
  local back_right_leg = Translate:new(leg, Vec:new(x - legs_half_distance, legs_height / 2, z - legs_half_distance))
  local front_left_leg = Translate:new(leg, Vec:new(x + legs_half_distance, legs_height / 2, z + legs_half_distance))
  local front_right_leg = Translate:new(leg, Vec:new(x + legs_half_distance, legs_height / 2, z - legs_half_distance))

  local map_size = 1
  local map_mat = Lambertian:new(Image:new("examples/images/planets/earth.jpg"))
  local map = Quad:new(Point:new(x - map_size / 2, surface_y, z + map_size / 2), Vec:new(map_size, 0, 0), Vec:new(0, 0, -map_size), map_mat)

  local jupiter = Sphere:new(Point:new(x - map_size / 2, surface_y + 0.3, z - map_size / 2), 0.3,
      Lambertian:new(Image:new("examples/images/planets/jupiter.jpg")))

  local lamp_cover_radius = 0.25
  local lamp_cover_height = 0.5
  local lamp_x = x
  local lamp_cover_mat = Lambertian:from_albedo(Color:new(1, 1, 1))
  local lamp_cover = Translate:new(
      RotateX:new(Cylinder:open(lamp_cover_radius, lamp_cover_height, lamp_cover_mat), -45),
      Vec:new(lamp_x, surface_y + 1, z - map_size))
  local lamp_bulb_mat = DiffuseLight:from_emission(Color:new(1.0, 0.84, 0.66))
  local lamp_bulb = Sphere:new(Point:new(lamp_x, surface_y + 1, z - map_size), 0.22, lamp_bulb_mat)

  objects:add_all(top, back_left_leg, back_right_leg, front_left_leg, front_right_leg, map, jupiter, lamp_cover, lamp_bulb)
end

local map_size = 1
local map_mat = Lambertian:new(Image:new("examples/images/planets/venus_surface.jpg"))
local map = Quad:new(Point:new(4.5 - map_size / 2, 0, 1.5), Vec:new(map_size, 0, 0), Vec:new(0, 0, -map_size), map_mat)
objects:add(map)

local function setup_camera()
  local camera = engine.Camera:new(1200, 16 / 9)
  camera.samples_per_pixel = 2000
  camera.max_depth = 50

  camera.field_of_view = 20
  camera.look_from = Point:new(13, 2, 3)
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

setup_bvh()

return engine.Scene:new(setup_camera(), objects)