temple = {
  _filters = {},
  _functions = {},
  _tests = {},

  addfilter = function(self, name, func)
    self._filters[name] = func
  end,

  addfunction = function(self, name, func)
    self._functions[name] = func
  end,

  addtest = function(self, name, func)
    self._tests[name] = func
  end
}

--- User code is appended below this line.

temple:addfilter("concat2", function(s1, s2)
  return ("CONCAT2: %s"):format(s1 .. s2)
end)
