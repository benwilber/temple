local filters = require "filters"
temple:addfilter("concat", filters.concat)
local s = temple:callfilter("concat", "ABC", "123")
print(s)

for k, v in pairs(temple.filters) do
    print(k, " ", v)
end
