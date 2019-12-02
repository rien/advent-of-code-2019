#!/usr/bin/julia

function fuel_needed(mass)
    div(mass, 3) - 2
end

sum = 0
for line in eachline("input")
    fuel = fuel_needed(parse(UInt64, line))
    fuel_sum = fuel
    while fuel > 8
        fuel = fuel_needed(fuel)
        fuel_sum += fuel
    end
    global sum += fuel_sum
end



write(stdout, string(sum))
