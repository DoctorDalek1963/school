	// A simple program to iteratively add two numbers, effectively multiplying them
	// Take a number, subtract 1, and store it as num1
	inp
	// We take 1 because it's actually easier to keep it rather than loading a 0
	sub one
	sta num1
	// Take a number and store it as orig and num2
	inp
	sta orig
	sta num2
	// While num1 > 0, num2 = num2 + orig
loop add orig
	sta num2
	lda num1
	sub one
	brz end // If acc == 0, just output num2
	// Else, store the new num1 and keep looping
	sta num1
	lda num2
	bra loop
end lda num2
	out
	hlt

num1    dat
num2    dat
orig    dat
one     dat 1
