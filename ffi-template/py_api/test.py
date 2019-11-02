from functools import wraps
import json
from ctypes import c_char_p


class Lib:
    def __init__(self, src: str):
        self.f = open(src, 'r').read()

    def free_string(self, s):
        print("Dropped " + str(s))

class Box:
    def __init__(self, ptr, free, lib):
        self._as_parameter_ = ptr
        self.free = free
        self.lib = lib

    @classmethod
    def from_param(cls, c):
        if isinstance(c, cls):
            return c
        else:
            raise TypeError('Wrong type expected ' + str(cls) + ' got ' + str(type(c)))

    def __del__(self):
        if not self.free is None:
            self.free(self.lib, self._as_parameter_)

libs = {}

def to_decorator(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        @wraps(func)
        def inner(inner_func):
            return func(*args, **kwargs, func=inner_func)
        return inner
    return wrapper

@to_decorator
def use_lib(lib_cls, src: str, func=None):
    # Connect the library
    if src not in libs:
        libs[src] = lib_cls(src)
    @wraps(func)
    def wrap(*args, **kwargs):
        # Parse the library to the function
        return func(*args, lib=libs[src], **kwargs)
    return wrap

@to_decorator
def map_arg(mapping, mapped=None, func=None):
    @wraps(func)
    def wrap(*args, **kwargs):
        # Convert args
        if mapped is None:
            nargs = list(map(lambda a: mapping(a[0]), zip(args, func.__code__.co_varnames)))
            nkwargs = dict(map(lambda a: (a[0], mapping(a[1])), kwargs.items()))
        else:
            nargs = list(map(lambda a: mapping(a[0]) if a[1] in mapped else a[0], zip(args, func.__code__.co_varnames)))
            nkwargs = dict(map(lambda a: (a[0], mapping(a[1])) if a[0] in mapped else a, kwargs.items()))
        # Do action
        return func(*nargs, **nkwargs)
    return wrap

@to_decorator
def map_res(mapping, func=None):
    @wraps(func)
    def wrap(*args, **kwargs):
        return mapping(func(*args, **kwargs))
    return wrap

@to_decorator
def map_free_res(free_func, func=None):
    @wraps(func)
    def wrap(*args, lib=None, **kwargs):
        return free_func(lib, func(*args, **kwargs))
    return wrap

@to_decorator
def map_box_res(init, free_func, func=None):
    @wraps(func)
    def wrap(*args, lib=None, **kwargs):
        return init(func(*args, **kwargs), free_func, lib)
    return wrap

@use_lib(Lib, 'f.txt')
@map_arg(lambda v: str(v)) # Map a to JsonObject
@map_box_res(Box, Lib.free_string)
def my_func(a, b=False, lib=None) -> Box:
    return a

if __name__ == '__main__':
    a = my_func(1, 2, 3)
    del a
