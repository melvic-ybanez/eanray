local ObjectList = engine.ObjectList
local Lambertian = engine.materials.Lambertian
local DiffuseLight = engine.materials.DiffuseLight
local Dielectric = engine.materials.Dielectric
local Metal = engine.materials.Metal
local Color = engine.Color
local Box = engine.shapes.Box
local Quad = engine.shapes.Quad
local Sphere = engine.shapes.Sphere
local ConstantMedium = engine.shapes.ConstantMedium
local Point = engine.math.Point
local Vec = engine.math.Vec

local boxes1 = ObjectList:new()
local ground = Lambertian:from_albedo(Color:new(0.48, 0.83, 0.53))

local boxes_per_side = 20

for i = 0, boxes_per_side - 1 do
  for j = 0, boxes_per_side - 1 do
    local w = 100.0
    local x0 = -1000.0 + i * w
    local z0 = -1000.0 + j * w
    local y0 = 0.0
    local x1 = x0 + w
    local y1 = engine.math.random_range(1, 101)
    local z1 = z0 + w

    boxes1:add(Box:new(Point:new(x0, y0, z0), Point:new(x1, y1, z1), ground))
  end
end

local world = ObjectList:new()

world:add(engine.BVH:new(boxes1))

local light = DiffuseLight:from_emission(Color:new(7, 7, 7))
world:add(Quad:new(Point:new(123, 554, 147), Vec:new(300, 0, 0), Vec:new(0, 0, 265), light))

local center1 = Point:new(400, 400, 200)
local center2 = center1 + Vec:new(30, 0, 0)
local sphere_material = Lambertian:from_albedo(Color:new(0.7, 0.3, 0.1))
world:add(Sphere:moving(center1, center2, 50, sphere_material))

world:add(Sphere:stationary(Point:new(260, 150, 45), 50, Dielectric:new(1.5)))
world:add(Sphere:stationary(Point:new(0, 150, 145), 50, Metal:new(Color:new(0.8, 0.8, 0.9), 1.0)))

local boundary = Sphere:stationary(Point:new(360, 150, 145), 70, Dielectric:new(1.5))
world:add(boundary)
world:add(ConstantMedium:from_albedo(boundary, 0.2, Color:new(0.2, 0.4, 0.9)))
boundary = Sphere:stationary(Point:new(0, 0, 0), 5000, Dielectric:new(1.5))
world:add(ConstantMedium:from_albedo(boundary, .0001, Color:new(1, 1, 1)))

local emat = Lambertian:from_texture(engine.textures.Image:new("examples/images/earthmap.jpg"))
world:add(Sphere:stationary(Point:new(400, 200, 400), 100, emat))
local pertext = engine.textures.Noise:new(0.2, Color:new(0.5, 0.5, 0.5))
world:add(Sphere:stationary(Point:new(220, 280, 300), 80, Lambertian:from_texture(pertext)))

local boxes2 = ObjectList:new()
local white = Lambertian:from_albedo(Color:new(.73, .73, .73))

local ns = 1000

for j = 0, ns - 1 do
  boxes2:add(Sphere:stationary(Point.random_range(0, 165), 10, white))
end

world:add(engine.BVH:new(boxes2):rotate_y(15):translate(-100, 270, 395))

-- change the width to 400, the spp to 250 and the max depth to 4, if you want to render a
-- lower quality image
local cam = engine.Camera:new(800, 1.0)

cam.aspect_ratio = 1.0
cam.samples_per_pixel = 10000
cam.max_depth = 40
cam.background = engine.Background:from_color(Color:new(0, 0, 0))

cam.field_of_view = 40
cam.look_from = Point:new(478, 278, -600)
cam.look_at = Point:new(278, 278, 0)
cam.vup = Vec:new(0, 1, 0)

cam.defocus_angle = 0

return engine.Scene:new(cam, world)