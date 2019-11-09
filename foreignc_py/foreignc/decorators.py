from typing import *
from functools import wraps
from .classes import Box, get_lib, BaseLib

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
def use_lib(name: str, func=None):
    # Connect the library
    @wraps(func)
    def wrap(*args, **kwargs):
        # Parse the library to the function
        return func(*args, lib=get_lib(name), **kwargs)
    return wrap

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

@to_decorator
def map_res(mapping: Callable[[T], TRes], func=None):
    @wraps(func)
    def wrap(*args, **kwargs):
        return mapping(func(*args, **kwargs))
    return wrap

@to_decorator
def map_free_res(free_func: Callable[[BaseLib, T], TRes], func=None):
    @wraps(func)
    def wrap(*args, lib=None, **kwargs):
        return free_func(lib, func(*args, **kwargs))
    return wrap

@to_decorator
def box_res(free_func: Callable[[BaseLib, T], TRes], func=None):
    @wraps(func)
    def wrap(*args, lib=None, **kwargs):
        return Box(func(*args, **kwargs), free_func, lib)
    return wrap
