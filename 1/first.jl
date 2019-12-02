#!/usr/bin/julia

sum = 0
for line in eachline("input")
    global sum += div(parse(UInt64, line), 3) - 2
end

write(stdout, string(sum))
