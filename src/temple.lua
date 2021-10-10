temple = {
  _filters = {},

  addfilter = function(self, name, func)
    self._filters[name] = func
  end
}

--- User code is appended below this line.

temple:addfilter("concat2", function(s1, s2)
  return ("CONCAT2: %s"):format(s1 .. s2)
end)
