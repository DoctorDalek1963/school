// Take 2 numbers as input and store them
inp
sta n1
inp
sta n2
// Echo the third number immediately
inp
out
// Echo the other numbers
lda n2
out
lda n1
out
hlt

n1 dat
n2 dat
