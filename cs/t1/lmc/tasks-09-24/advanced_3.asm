// Take two numbers and multiply them
inp
sub one
sta count

inp
sta orig
sta num

// While count > 0, num = num + orig
_loop add orig
sta num

lda count
sub one
brz _end // If acc == 0, just output num

// Else, store the new count and keep looping
sta count
lda num
bra _loop

_end lda num
out
hlt

count dat
num dat
orig dat
one dat 1
