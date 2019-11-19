import json, os, sys
from ctypes import *

class FFiError(ConnectionError):
    def __init__(self, message):
        super().__init__(message)

class BaseLib:
    def __init__(self, src: str):
        self.__lib__ = cdll.LoadLibrary(os.path.abspath(src))

class LibValue:
    def __init__(self, lib):
        self.__lib__ = lib

    @staticmethod
    def __ty__():
        return None

    @classmethod
    def __from_result__(cls, res, lib):
        return None

    def __free__(self, lib):
        pass

    def __del__(self):
        if hasattr(self, '__lib__') and self.__lib__ is not None:
            self.__free__(self.__lib__)

    def __validate__(self):
        return None

class LibString(LibValue):
    @staticmethod
    def __ty__():
        return c_char_p

    @classmethod
    def __from_result__(cls, res, lib):
        bytes = res.value
        if bytes is not None:
            lib.free_string(res)
            return bytes.decode('utf-8')
        else:
            return None

    def from_param(v):
        if isinstance(v, str):
            return c_char_p(v.encode('utf-8'))
        raise ArgumentError('Wrong argument expected a string got ' + str(type(v)))

class Box(LibValue):
    def __init__(self, res, lib):
        super().__init__(lib)
        self.__value__ = res

    @staticmethod
    def __ty__():
        return c_void_p

    @classmethod
    def __from_result__(cls, res, lib):
        return cls(res, lib)

    def __free__(self, lib):
        print(self)
        getattr(lib, self.__free_func__())(self.__value__)

    @staticmethod
    def __free_func__() -> str:
        raise NotImplementedError()

    def from_param(v):
        if isinstance(v, Box):
            return v.__value__
        raise ArgumentError('Expected box ' + str(type(v)))

class Json(LibValue):
    def __init__(self, res, lib):
        super().__init__(lib)
        if isinstance(res, str):
            self.__json__ = res
        else:
            self.__json__ = json.dumps(res)

    @staticmethod
    def __ty__():
        return LibString.__ty__()

    @classmethod
    def __from_result__(cls, res, lib):
        return cls(LibString.__from_result__(res, lib), lib)

    @property
    def value(self):
        return self.__json__

    @value.setter
    def value(self, s: str):
        self.__json__ = s

    @property
    def object(self):
        return json.loads(self.__json__)

    @object.setter
    def object(self, v: str):
        self.__json__ = json.dumps(v)

    def from_param(v):
        if isinstance(v, Json):
            return c_char_p(v.value.encode('utf-8'))
        raise ArgumentError('Expected Json ' + str(type(v)))


def to_c_type(ty):
    if ty == int:
        return c_int
    if ty == float:
        return c_float
    if ty == bool:
        return c_bool
    if ty == str:
        return LibString
    if isinstance(ty, LibValue):
        return LibValue
    return ty

def read_pointer(cls, ptr, lib):
    if ptr and ptr != 1:
        content = ptr.contents
        if hasattr(cls, '__from_result__'):
            return cls.__from_result__(content, lib)
        else:
            return content
    else:
        return None

results = {}
def RESULT(T, E):
    if (T, E) in options:
        return options[(T, E)]

    if T is None or E is None:
        raise ArgumentError("Result types cannot be none T: " + str(T) + " E: " + str(E))
    ok_ty = to_c_type(T)
    err_ty = to_c_type(E)

    class CResult(Structure):
        _fields_ = [
            ("ok", POINTER(ok_ty if not hasattr(ok_ty, '__ty__') else ok_ty.__ty__())),
            ("err", POINTER(err_ty if not hasattr(err_ty, '__ty__') else err_ty.__ty__()))
        ]

    class Result(LibValue):
        def __init__(self, ok, err, lib):
            super().__init__(lib)
            self.ok = ok
            self.err = err

        @staticmethod
        def __ty__():
            return POINTER(CResult)

        @classmethod
        def __from_result__(cls, res, lib):
            if res:
                result = res.contents
                ok = read_pointer(ok_ty, result.ok, lib)
                err = read_pointer(err_ty, result.err, lib)
                return Result(ok, err, lib)
            else:
                raise FFiError("Recieved null pointer as base result")

        def from_param(v):
            raise NotImplementedError('Errors as arguments are not supported')

        def get_ok(self) -> T:
            return self.ok

        def get_err(self) -> E:
            return self.err

        def is_ok(self) -> bool:
            return self.ok is not None

        def is_err(self) -> bool:
            return self.err is not None

        def consume(self) -> T:
            if self.err is not None:
                raise FFiError(self.err)
            return self.ok

    results[(T, E)] = Result
    return Result

options = {}
def OPTION(T):
    if T in options:
        return options[T]

    if T is None:
        raise ArgumentError("Option type cannot be none T: " + str(T))

    ty = to_c_type(T)
    ptr = POINTER(ty if not hasattr(ty, '__ty__') else ty.__ty__())

    class Option(LibValue):

        @staticmethod
        def __ty__():
            return ptr

        def __init__(self, v: T, lib):
            super().__init__(lib)
            self.__value__ = v

        @classmethod
        def __from_result__(cls, res, lib):
            inner = read_pointer(ptr, res, res)
            if inner is not None and hasattr(ty, '__from_result__'):
                return Option(ty.__from_result__(inner, lib), lib)
            else:
                return Option(inner, lib)

        def from_param(v):
            if isinstance(v, Option):
                return Option(v.__value__, None)
            if v is None:
                return Option(None, None)
            if isinstance(v, ty):
                return Option(v, None)
            if hasattr(ty, 'from_param'):
                return Option(v, None)
            raise ArgumentError('Expected ' + str(ty) + ' but got ' + str(type(v)))

        def unwrap(self) -> T:
           return self.__value__

    options[T] = Option
    return Option

types = {}

def submit_type(name, ty):
    types[name] = ty

def get_type(ty):
    if ty in types:
        return types[ty]
    return ty