#!/usr/bin/env julia

# Print the sum of multiples of 3 or 5 less than the argument
println(sum([x for x in 1:(parse(Int, ARGS[1]) - 1) if x % 3 == 0 || x % 5 == 0]))
