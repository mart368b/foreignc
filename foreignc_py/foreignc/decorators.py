from typing import *
from functools import wraps
from .classes import LibValue, RESULT, FFiError
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
    def wrapper(*args, **kwargs):
        if func.__code__.co_varnames[0] == 'self':
            lib = args[0].__lib__
        elif func.__code__.co_varnames[0] == 'lib':
            lib = args[0].__lib__
        else:
            raise FFiError("Missing self or library reference for function " + str(func))
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
            abi_func = getattr(lib, name)
            abi_func.argtypes = argtypes
            abi_func.restype = RESULT(restype, str)
            abi_func.errcheck = apply_lib_value(lib, errcheck)
        return func(*args, **kwargs).consume()
    return wrapper

def apply_lib_value(lib, errcheck = None):
    def inner(r, *args, **kwargs):
        if isinstance(r, LibValue):
            r = r.__into__(lib)
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