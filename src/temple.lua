--[[
This Lua script implements the plugin runtime of the temple program.

It defines a single global table, `temple`, that has methods and attributes
for registering filters and callbacks, fine-grained control of autoescaping,
and more.

The program tries to minimize (as much as possible) the number of places where
Lua values have to cross the boundary into Rust values.  For the most part,
we try to keep Lua stuff on the Lua side, and Rust stuff on the Rust side.  The
main exception to this rule is values given to custom template filter functions
that are implemented in Lua.  
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
