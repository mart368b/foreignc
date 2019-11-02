from functools import wraps
import json


class Lib:
    def __init__(self, src: str):
        self.f = open(src, 'r').read()

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
def map_arg(mapping, j_args=None, j_kwarg=None, res=False, func=None):
    @wraps(func)
    def wrap(*args, **kwargs):
        print(args)
        # Convert args
        if not j_args is None:
            nargs = list(map(lambda a: mapping(a[1]) if a[0] in j_args else a[1], enumerate(args)))
        else: nargs = args;
        # Convert kwargs
        if not j_kwarg is None:
            nkwargs = dict(map(lambda a: (a[0], mapping(a[1])) if a[0] in j_kwarg else a, kwargs.items()))
        else: nkwargs = kwargs;
        # Do action
        r = func(*nargs, **nkwargs)
        # Convert result
        if res: return mapping(r);
        else: return r;
    return wrap



@map_arg(json.dumps, j_args=[0]) # Map a to JsonObject
def my_func(a, b="", lib=None):
    print(a)
    print(b)
    print(lib)

if __name__ == '__main__':
    my_func('{"a":"b"}', b='{"c":"d"}')
