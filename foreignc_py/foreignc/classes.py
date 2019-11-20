import json, os, sys
from ctypes import *

class GenericError(TypeError):
    def __init__(self, message):
        super().__init__(message)

class FFiError(ConnectionError):
    def __init__(self, message):
        super().__init__(message)

class BaseLib:
    def __init__(self, src: str):
        self.__lib__ = cdll.LoadLibrary(os.path.abspath(src))

class LibValue:
    def __init__(self, lib):
        self.__lib__ = lib

    @classmethod
    def __ty__(cls):
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
    @classmethod
    def __ty__(cls):
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
    def value(self) -> str:
        return self.__json__

    @value.setter
    def value(self, s: str):
        self.__json__ = s

    @property
    def object(self) -> object:
        return json.loads(self.__json__)

    @object.setter
    def object(self, v: object):
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
class Result(LibValue):
    T = None
    E = None
    _CResult_ = None

    def __init__(self, ok, err, lib):
        super().__init__(lib)
        self.ok = ok
        self.err = err

    def __class_getitem__(cls, TT):
        Ty, Ey = TT
        if Ty == str:
            Ty = LibString
        if Ey == str:
            Ey = LibString
        if (Ty, Ey) in results:
            return results[(Ty, Ey)]

        if Ty is None and Ey is None:
            raise ArgumentError("Result types cannot be none T: " + str(Ty) + " E: " + str(Ey))

        ok_ty = to_c_type(Ty)
        err_ty = to_c_type(Ey)

        class CResult(Structure):
            _fields_ = [
                ("ok", POINTER(ok_ty if not hasattr(ok_ty, '__ty__') else ok_ty.__ty__())),
                ("err", POINTER(err_ty if not hasattr(err_ty, '__ty__') else err_ty.__ty__()))
            ]

        class TypedResult(Result):
            T = Ty
            E = Ey
            _CResult_ = CResult

        results[(Ty, Ey)] = TypedResult
        return TypedResult

    @classmethod
    def __ty__(cls):
        return POINTER(cls._CResult_)

    @classmethod
    def __from_result__(cls, res, lib):
        if res:
            result = res.contents
            ok = read_pointer(to_c_type(Result.T), result.ok, lib)
            err = read_pointer(to_c_type(Result.E), result.err, lib)
            return cls(ok, err, lib)
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


options = {}
class Option(LibValue):
    T = None
    _Pointer_ = None

    def __init__(self, v: T, lib = None):
        super().__init__(lib)
        self.__value__ = v

    def __class_getitem__(cls, Ty):
        if Ty == str:
            Ty = LibString
        if Ty in options:
            return options[Ty]

        if Ty is None:
            raise ArgumentError("Option type cannot be none T: " + str(Ty))
        ty = to_c_type(Ty)

        class TypedOption(Option):
            T = Ty
            _Pointer_ = POINTER(ty if not hasattr(ty, '__ty__') else ty.__ty__())

        options[Ty] = TypedOption
        return TypedOption

    @classmethod
    def __from_result__(cls, res, lib):
        inner = read_pointer(to_c_type(Option.T), res, lib)
        if inner is not None and hasattr(Option.T, '__from_result__'):
            return cls(inner, lib)
        else:
            return cls(inner, lib)

    @classmethod
    def __ty__(cls):
        return cls._Pointer_

    @classmethod
    def from_param(cls, v):
        if v is None:
            return cls._Pointer_()
        if isinstance(v, Option):
            inner = v.__value__
            if hasattr(cls.T, 'from_param'):
                inner = cls.T.from_param(inner)
            return cls._Pointer_(inner)
        if isinstance(v, cls.T):
            return False
        if not isinstance(cls.T, Option) and hasattr(cls.T, 'from_param'):
            return cls._Pointer_(cls.T.from_param(v))
        raise ArgumentError('Expected ' + str(cls.T) + ' but got ' + str(type(v)))

    def unwrap(self) -> T:
       return self.__value__

types = {}
def submit_type(name, ty):
    types[name] = ty

def get_type(ty):
    if ty in types:
        return types[ty]
    return ty