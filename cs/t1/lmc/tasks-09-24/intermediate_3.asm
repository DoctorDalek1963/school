// Take 2 numbers and output the biggest minus the smallest
inp
sta n1
inp
sta n2

// Sub n1 and if it's positive, just output that
sub n1
brp _output

// If n2 - n1 < 0, then n1 > n2
lda n1
sub n2

_output out
hlt

n1 dat
n2 dat
