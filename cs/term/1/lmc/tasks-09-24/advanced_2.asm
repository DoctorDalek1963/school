// Take a big number and a small number and subtract the smaller number until you get < 0, then print how many times you could subtract the number
// Integer division
inp
sta num
inp
sta smallNum

_loop lda num
sub smallNum
sta num // Keep the number

// Subtract one from the count
lda count
add one
sta count

// We load num so that we can check if it's positive
lda num

brp _loop

lda count
sub one // This just fixes an off-by-one error
out
hlt

num dat
count dat
one dat 1
smallNum dat
