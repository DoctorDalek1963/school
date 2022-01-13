// Take a big number and a small number and subtract the smaller number until you get < 0, then print that
inp
sta bigNum
inp
sta smallNum

lda bigNum
_loop sub smallNum
brp _loop
out
hlt

bigNum dat
smallNum dat
