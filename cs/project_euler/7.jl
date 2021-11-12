#!/usr/bin/env julia

"""
    isprime(n)

Find whether n is a prime number.
"""
function isprime(n::Int)::Bool
	n == 1 && return false

	for i in 2:(n // 2)
		n % i == 0 && return false
	end

	return true
end

function main()
	primes = [2, 3]
	i = 3

	while length(primes) <= 10001
		if isprime(i)
			push!(primes, i)
		end
		i += 2
	end

	println(primes[end])
end

if abspath(PROGRAM_FILE) == @__FILE__
	main()
end
