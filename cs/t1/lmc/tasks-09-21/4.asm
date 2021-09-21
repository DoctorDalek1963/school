// Take 2 numbers as input
inp
sta n1
inp
sta n2
// Do n1 - n2 and print it
lda n1
sub n2
out
// Do n2 - n1 and print input
lda n2
sub n1
out
hlt

n1 dat
n2 dat
