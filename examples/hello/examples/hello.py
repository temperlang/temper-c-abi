import contextlib as ctx
import ctypes as c
import pathlib as p
import sys as s


def greeting_for(name: str) -> str:
    with _string_mktemp(name) as name_string:
        return _string_to_str(_lib.hello_greeting_for(name_string))


def _string_make(s: str):
    return _lib.hello_String_new(s.encode())


@ctx.contextmanager
def _string_mktemp(s: str):
    hs = _string_make(s)
    try:
        yield hs
    finally:
        _lib.hello_String_free(hs)


def _string_to_str(hs):
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

# TODO Pull any config from some environmental context???
_lib.hello_init()
