from typing import *
from functools import wraps
from ctypes import POINTER
from .classes import LibValue, ARGRESULT
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
        invalid = []
        for arg in args:
            if isinstance(arg, LibValue):
                err = arg.__validate__()
                if err is not None:
                    invalid.append(err)
        if len(invalid) > 0:
            raise AssertionError('Recieved invalid arguments: \n    ' + '\n '.join(map(str, invalid)))

        nonlocal is_implemented
        if not is_implemented:
            is_implemented = True
            abi_func = getattr(self.__lib__, name)
            abi_func.argtypes = argtypes
            abi_func.restype = POINTER(ARGRESULT(restype))
            abi_func.errcheck = apply_lib_value(ref(self), errcheck)
        res = func(self, *args, **kwargs)
        return unwrap_err(res)
    return wrapper

def unwrap_err(v):
    print(v.contents.inner_value)
    return v

def apply_lib_value(lib, errcheck = None):
    def inner(r, *args, **kwargs):
        p = r
        while (p is not None and not isinstance(p, LibValue) and hasattr(p, '_type_')):
            print(type(p._type_))
            p = r.contents
        if isinstance(p, LibValue):
            r = p
            r = r.__into__(lib().__lib__)
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