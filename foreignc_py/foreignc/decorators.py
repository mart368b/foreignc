from typing import *
from functools import wraps
from ctypes import c_char_p
from .classes import LibValue
from weakref import ref

T = TypeVar('T')
TRes = TypeVar('TRes')

def to_decorator(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        @wraps(func)
        def inner(inner_func):
            return func(*args, **kwargs, func=inner_func)
        return inner
    return wrapper

@to_decorator
def create_abi(name: str, argtypes = (), restype = None, errcheck = None, func = None):
    if not isinstance(argtypes, tuple):
        raise TypeError('Expected tuple got ' + str(type(argtypes)))
    is_implemented = False
    @wraps(func)
    def wrapper(self, *args, **kwargs):
        nonlocal is_implemented
        if not is_implemented:
            is_implemented = True
            abi_func = getattr(self.__lib__, name)
            abi_func.argtypes = argtypes
            abi_func.restype = restype
            abi_func.errcheck = apply_lib_value(ref(self), errcheck)
        return func(self, *args, **kwargs)
    return wrapper

def apply_lib_value(lib, errcheck = None):
    def inner(r, *args, **kwargs):
        if isinstance(r, LibValue):
            r.__lib__ = lib().__lib__
            r = r.__into__()
        if errcheck is not None:
            res = errcheck(r, *args, **kwargs)
            return apply_lib_value(lib, None)(res, *args, **kwargs)
        else:
            return r
    return inner

@to_decorator
def map_arg(mapping: Callable[[T], TRes], mapped: Optional[List[str]]=None, func=None):
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