from functools import wraps
from .classes import LibValue, RESULT, FFiError, get_type
from ctypes import ArgumentError

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
            if args[0] == None:
                raise ArgumentError("Expected library got none")
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
            abi_func.argtypes = map(get_type, argtypes)
            abi_func.restype = RESULT(get_type(restype), str).__ty__()
            abi_func.errcheck = apply_lib_value(lib, RESULT(get_type(restype), str), errcheck)
        return func(*args, **kwargs).consume()
    return wrapper

def apply_lib_value(lib, res_ty, errcheck = None):
    def inner(r, *args, **kwargs):
        if hasattr(res_ty, '__from_result__'):
            r = res_ty.__from_result__(r, lib)
        if errcheck is not None:
            res = errcheck(r, *args, **kwargs)
            return apply_lib_value(lib, None)(res, *args, **kwargs)
        else:
            return r
    return inner