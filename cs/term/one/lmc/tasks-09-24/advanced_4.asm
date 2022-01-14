// Take two numbers and return the integer div and remainder
inp
sta n1

inp
sta n2

_loop lda n1
sub n2
brp _continue_loop

// If we've gone < 0, print the count and the old n1, then halt
lda count
out
lda n1
out
hlt

_continue_loop sta n1
lda count
add one
sta count

bra _loop

n1 dat
n2 dat
count dat 0
one dat 1
