local Color = engine.Color
local Point = engine.math.Point
local Vec = engine.math.Vec
local Lambertian = engine.materials.Lambertian
local Metal = engine.materials.Metal
local Dielectric = engine.materials.Dielectric
local Sphere = engine.shapes.Sphere
local ObjectList = engine.ObjectList
local textures = engine.textures
local Image = textures.Image

local objects = ObjectList:new()

local planets_dir = "examples/images/planets/"

local function make_ground()
  local radius = 1000
  local ground = Sphere:stationary(Point:new(0, -radius, 0), radius, Lambertian:new(textures.Noise:new(4)))
  objects:add(ground)
end

make_ground()

-- big spheres
objects:add(Sphere:stationary(Point:new(-4, 1, 0), 1.0, Lambertian:new(Image:new(planets_dir .. "jupiter.jpg"))))
objects:add(Sphere:stationary(Point:new(4, 1, 0), 1.0, Lambertian:new(Image:new("examples/images/moon.jpg"))))

local function make_group()
  local base_radius = 0.6
  local head_radius = 0.4
  local z = 2.5
  objects:add(Sphere:stationary(Point:new(3, base_radius, z), base_radius, Lambertian:new(Image:new(planets_dir .. "venus_surface.jpg"))))
  objects:add(Sphere:stationary(Point:new(3, base_radius * 2 + head_radius, z), head_radius, Metal:new(Color:new(0.7, 0.6, 0.5), 0)))
end

make_group()

local small_radius = 0.25
local bouncing_radius = 0.2

local function make_small_lambertian(center)
  local albedo = Color.random() * Color.random() + Color:new(0.1, 0.2, 0)
  objects:add(Sphere:stationary(center, small_radius, Lambertian:from_albedo(albedo)))
end

local function make_small_metal(center)
  local albedo = Color.random_range(0.5, 1)
  local fuzz = engine.math.random_range(0, 0.5)
  objects:add(Sphere:stationary(center, small_radius, Metal:new(albedo, fuzz)))
end

local function make_small_glass(center)
  local sphere_material = Dielectric:new(Dielectric.RefractiveIndex.GLASS)
  objects:add(Sphere:stationary(center, small_radius, sphere_material))
end

local function make_small_moving_sphere(center, horizontal)
  local albedo = Color.random() * Color.random()
  local moving_comp = engine.math.random_range(0.1, 0.25)
  local moving_vec = Vec:new(0, moving_comp, 0)
  if horizontal then
    -- we are moving the z component instead of x to consider the angle we are looking from
    moving_vec = Vec:new(0, 0, moving_comp)
  end
  local center2 = center + moving_vec
  objects:add(Sphere:moving(center, center2, bouncing_radius, Lambertian:from_albedo(albedo)))
end

local function make_small_planet(center, filename)
  objects:add(Sphere:stationary(center, small_radius, Lambertian:new(Image:new(planets_dir .. filename))))
end

make_small_planet(Point:new(5.5, small_radius, 0), "mars.jpg")
make_small_planet(Point:new(2.5, small_radius, 1), "venus_atmosphere.jpg")
make_small_planet(Point:new(5, small_radius, 2), "earth.jpg")
make_small_glass(Point:new(4, small_radius, 3.3))
make_small_lambertian(Point:new(2, small_radius, -2.5))
make_small_lambertian(Point:new(-3.8, small_radius, 3.3))
make_small_lambertian(Point:new(-3.5, small_radius, 2.5))
make_small_metal(Point:new(-4.1, small_radius, 1.4))

make_small_moving_sphere(Point:new(6.5, bouncing_radius, -0.7), false)
make_small_moving_sphere(Point:new(-0.5, bouncing_radius, 1), true)
make_small_moving_sphere(Point:new(7.2, bouncing_radius, 3.2), true)

objects:add(Sphere:stationary(Point:new(5.6, 0.2, 2.7), 0.2, Lambertian:from_albedo(Color.random() * Color.random())))
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

local world = engine.BVH:new(objects)
objects = ObjectList:new()
objects:add(world)

return engine.Scene:new(camera, objects)