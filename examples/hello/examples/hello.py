import contextlib as ctx
import ctypes as c
import pathlib as p
import sys as s
import weakref as w


def greeting_for(name: str) -> str:
    return str(String(_lib.hello_greeting_for(String.from_str(name).value)))


class String:
    def from_str(s: str):
        return String(_lib.hello_String_new(s.encode()))

    def __init__(self, value):
        self.value = value
        w.finalize(self, _lib.hello_String_free, value)

    def __str__(self):
        length: int = _lib.hello_String_length(self.value) + 1
        buffer = c.create_string_buffer(length)
        _lib.hello_String_copy(self.value, buffer, length)
        return buffer.value.decode()


def _choose_lib_name(base: str) -> str:
    if s.platform.startswith("win"):
        lib_name = f"{base}.dll"
    elif s.platform.startswith("darwin"):
        lib_name = f"lib{base}.dylib"
    else:
        lib_name = f"lib{base}.so"
    return lib_name


_lib_dir = p.Path(__file__).parent.parent / "target" / "debug"
_lib_path = _lib_dir / _choose_lib_name("hello")
_lib = c.CDLL(str(_lib_path))

_lib.hello_String_new.argtypes = [c.c_char_p]
_lib.hello_String_new.restype = c.c_void_p

_lib.hello_String_length.argtypes = [c.c_void_p]
_lib.hello_String_length.restype = c.c_size_t

_lib.hello_String_copy.argtypes = [c.c_void_p, c.c_char_p, c.c_size_t]
_lib.hello_String_copy.restype = c.c_size_t

_lib.hello_String_free.argtypes = [c.c_void_p]
_lib.hello_String_free.restype = None

_lib.hello_greeting_for.argtypes = [c.c_void_p]
_lib.hello_greeting_for.restype = c.c_void_p

_lib.hello_init.argtypes = []
_lib.hello_init.restype = None

# TODO Pull any config from some environmental context???
_lib.hello_init()
