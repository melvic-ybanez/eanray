local Color = engine.Color
local Point = engine.math.Point
local Lambertian = engine.materials.Lambertian
local Metal = engine.materials.Metal
local Dielectric = engine.materials.Dielectric
local Sphere = engine.shapes.Sphere

local function make_big_3_spheres(objects)
  objects:add_all(
      Sphere:stationary(Point:new(0, 1, 0), 1.0, Dielectric:new(Dielectric.RefractiveIndex.GLASS)),
      Sphere:stationary(Point:new(-4, 1, 0), 1.0, Lambertian:from_albedo(Color:new(0.4, 0.2, 0.1))),
      Sphere:stationary(Point:new(4, 1, 0), 1.0, Metal:new(Color:new(0.7, 0.6, 0.5), 0))
  )
end

return {
  make_big_3_spheres = make_big_3_spheres
}