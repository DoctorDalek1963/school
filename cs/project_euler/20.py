#!/usr/bin/env python

def fac(n: int) -> int:
    if n in (0, 1):
        return 1

    return n * fac(n - 1)


def main() -> None:
    x = fac(100)
    print(sum([int(n) for n in str(x)]))


if __name__ == '__main__':
    main()
