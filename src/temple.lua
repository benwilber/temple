--[[
This Lua script implements the plugin runtime of the temple program.

It defines a single global table, `temple`, that has methods and attributes
for registering filters and callbacks, fine-grained control of auto-escaping,
and more.
--]]
temple = {
    filters = {},

    addfilter = function(self, name, func)
        self.filters[name] = func
    end,

    callfilter = function(self, name, ...)
        local func = self.filters[name]

        if func then
            return func(...)
        else
            error("unknown filter: " .. name)
        end
    end
}

-- The user's script is appended below this line before executing
