#!/usr/bin/env julia
println(sum([x for x in 1:(parse(Int, ARGS[1]) - 1) if x % 3 == 0 || x % 5 == 0]))
