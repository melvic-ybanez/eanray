engine.ObjectList.add_all = function(self, ...)
  local objects = {...}
  for _, object in ipairs(objects) do
    self:add(object)
  end
end