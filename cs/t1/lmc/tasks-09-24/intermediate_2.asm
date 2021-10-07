// Take 2 numbers, and output the biggest, then the smallest
inp
sta n1
inp
sta n2

// Check if n2 >= n1
sub n1
brp _n2gteq

// If n2 < n1 (n2 - n1 < 0)
lda n1
out
lda n2
out
hlt

// If n2 >= n1
_n2gteq lda n2
out
lda n1
out
hlt

n1 dat
n2 dat
