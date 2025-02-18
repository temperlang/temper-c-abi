import ctypes as c
import pathlib as p
import sys as s


def choose_lib_name(base: str) -> str:
    if s.platform.startswith("win"):
        lib_name = f"{base}.dll"
    elif s.platform.startswith("darwin"):
        lib_name = f"lib{base}.dylib"
    else:
        lib_name = f"lib{base}.so"
    return lib_name


lib_dir = p.Path(__file__).parent.parent / "target" / "debug"
lib_path = lib_dir / choose_lib_name("hello")
hello_lib = c.CDLL(str(lib_path))

print(hello_lib)
print(hello_lib.hello_String_new)
