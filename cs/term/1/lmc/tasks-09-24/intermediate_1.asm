// Take 2 numbers. If they're the same, print double the numbder. Else, print each separately

inp
sta n1
inp
sta n2

// Check if n1 == n2
sub n1
brz _same

// If n1 != n2
lda n1
out
lda n2
out
hlt

// Else (n1 == n2)
_same lda n1
add n2
out
hlt

n1 dat
n2 dat
