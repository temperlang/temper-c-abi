import hello as h
import ctypes as c
import pathlib as p
import sys as s


def init():
    # TODO Init the ctypes dll stuff here also?
    _lib.hello_init()


# def greeting_for(name: str) -> str:
#     name_string = _lib.hello_String_new(name.encode())
#     try:
#         return _string_value(_lib.hello_greeting_for(name_string))
#     finally:
#         _lib.hello_String_free(name_string)


def greeting_for(name: str) -> str:
    with String.from_str(name) as name_string:
        with String(_lib.hello_greeting_for(name_string.value)) as greeting:
            return str(greeting)


class String:
    def from_str(s: str):
        return String(_lib.hello_String_new(s.encode()))

    def __init__(self, value):
        self.value = value

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        _lib.hello_String_free(self.value)

    def __str__(self):
        length: int = _lib.hello_String_length(self.value) + 1
        buffer = c.create_string_buffer(length)
        _lib.hello_String_copy(self.value, buffer, length)
        return buffer.value.decode()


def _string_value(hs):
    try:
        length: int = _lib.hello_String_length(hs) + 1
        buffer = c.create_string_buffer(length)
        _lib.hello_String_copy(hs, buffer, length)
        return buffer.value.decode()
    finally:
        _lib.hello_String_free(hs)


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
