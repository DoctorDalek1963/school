// Take a number and store it
inp
sta n
// Take a number and add the stored number
inp
add n
// Store the result
sta n
// Take a third number and add it to the old result
inp
add n
out
hlt

n dat
