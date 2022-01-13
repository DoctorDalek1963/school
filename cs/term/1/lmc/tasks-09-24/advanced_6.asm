// Task 6 is just improving your solution to task 5, so I've put them together
// Take two numbers and check if one is a factor of the other
inp
sta n1
inp
sta n2

// Make sure we know which one is bigger
sub n1
brp _n2gteq
// If n2 < n1, then n2 - n1 < 0, so big = n1, small = n2
lda n1
sta big
lda n2
sta small
bra _loop

// If n2 >= n1, the big = n2, small = n1
_n2gteq lda n2
sta big
lda n1
sta small

_loop lda big
sub small
brz _factor
sta big
brp _loop

// If we've gone < 0, print -1 to show not factors
lda negOne
out
hlt

_factor out // If it's a factor, then we have 0 in the acc
hlt

n1 dat
n2 dat
big dat
small dat
negOne dat -1
