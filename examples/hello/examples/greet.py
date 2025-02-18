import hello as h
import ctypes as c
import pathlib as p
import sys as s


def main():
    h.init()
    print(h.greeting_for("world"))


if __name__ == "__main__":
    main()
