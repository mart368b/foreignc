import json, os, sys
from ctypes import *
from weakref import ref

class GenericError(Exception):
    def __init__(self, message):
        super().__init__(message)

class FFiError(Exception):
    def __init__(self, message):
        super().__init__(message)

class ArgumentError(Exception):
    def __init__(self, message):
        super().__init__(message)

class BaseLib:
    def __init__(self, src: str):
        self.__lib__ = cdll.LoadLibrary(os.path.abspath(src))

class LibValue:
    def __init__(self, lib):
        self.__lib__ = lib
        self.__resources__ = []

    @classmethod
    def __ty__(cls):
        return None

    @classmethod
    def __from_result__(cls, res, lib):
        return None

    def __new_resource__ (self, res):
        self.__resources__.append(res)

    def __free__(self, lib):
        for name, res in self.__resources__:
            getattr(lib, name)(res)

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
        for name, res in self.__resources__:
            getattr(lib, name)(res)

    @staticmethod
    def __free_func__() -> str:
        raise NotImplementedError()

    def from_param(v):
        if isinstance(v, Box):
            return v.__value__
        raise ArgumentError('Expected box ' + str(type(v)))

class RawPointer(LibValue):
    def __init__(self, res, lib):
        super().__init__(lib)
        self.__value__ = res

    @staticmethod
    def __ty__():
        return c_void_p

    @classmethod
    def __from_result__(cls, res, lib):
        return cls(res, lib)

    def from_param(v):
        if isinstance(v, RawPointer):
            return v.__value__
        raise ArgumentError('Expected raw pointer got ' + str(type(v)))

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

    def __str__(self):
        return self.__json__

    @classmethod
    def from_param(cls, v):
        if isinstance(v, Json):
            v.buff = v.value.encode('utf-8')
            return c_char_p(v.buff)
        raise ArgumentError('Expected Json ' + str(type(v)))

def to_c_type(ty):
    if ty == str:
        return LibString
    if ty == int:
        return c_int
    if ty == float:
        return c_float
    if ty == bool:
        return c_bool
    return ty

def read_pointer(cls, ptr, lib):
    if ptr and ptr != 1:
        content = ptr.contents
        if hasattr(cls, '__from_result__'):
            return cls.__from_result__(content, lib)
        else:
            if hasattr(content, 'value'):
                return content.value
            else:
                return content
    else:
        return None

results = {}
class Result(LibValue):
    T = None
    E = None
    CT = None
    CE = None
    _CResult_ = None

    def __init__(self, is_err, value, lib = None, raw = None):
        super().__init__(lib)
        self.__err__ = is_err
        self.__value__ = value
        self.__raw__ = raw
        if isinstance(value, LibValue):
            if raw is not None:
                value.__new_resource__(('free_cresult', raw))
            self.__raw__ = None
            for res in self.__resources__:
                value.__new_resource__(res)

    def __class_getitem__(cls, TT):
        Ty, Ey = TT
        if Ty is None and Ey is None:
            raise ArgumentError("Result types cannot be none T: " + str(Ty) + " E: " + str(Ey))

        if (Ty, Ey) in results:
            return results[(Ty, Ey)]

        ok_ty = to_c_type(Ty)
        err_ty = to_c_type(Ey)

        class CResult(Structure):
            _fields_ = [
                ("is_err", c_bool),
                ("value", POINTER(c_int)),
            ]

        class TypedResult(Result):
            T = Ty
            E = Ey
            CT = ok_ty
            CE = err_ty
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
            if result.is_err:
                ptr = cast(result.value, POINTER(cls.CE if not hasattr(cls.CE, '__ty__') else cls.CE.__ty__()))
            else:
                ptr = cast(result.value, POINTER(cls.CT if not hasattr(cls.CT, '__ty__') else cls.CT.__ty__()))
            value = read_pointer(cls.CT, ptr, lib)
            return cls( result.is_err, value, lib, raw=ptr)
        else:
            raise FFiError("Recieved null pointer as base result")

    def __free__(self, lib):
        if self.__raw__ is not None and len(self.__resources__) is not 0:
            print('Free result')
            lib.free_cresult(self.__raw__)
        for name, res in reversed(self.__resources__):
            getattr(lib, name)(res)

    def __new_resource__ (self, res):
        if isinstance(self.__value__, LibValue):
            self.__value__.__new_resource__(res)
        else:
            self.__resources__.append(res)

    def from_param(v):
        raise NotImplementedError('Errors as arguments are not supported')

    def get_ok(self) -> T:
        return self.__value__ if self.is_ok() else None

    def get_err(self) -> E:
        return None if self.is_ok() else self.__value__

    def is_ok(self) -> bool:
        return not self.__err__

    def is_err(self) -> bool:
        return self.__err__

    def consume(self) -> T:
        if self.is_err():
            raise FFiError(self.__value__)
        return self.__value__

options = {}
class Option(LibValue):
    T = None
    CT = None
    _Pointer_ = None

    def __init__(self, value: T, lib = None, raw = None):
        super().__init__(lib)
        self.__value__ = value
        self.__raw__ = raw
        if isinstance(value, LibValue):
            if raw is not None:
                value.__new_resource__(('free_coption', raw))
            self.__raw__ = None
            for res in self.__resources__:
                value.__new_resource__(res)

    def __class_getitem__(cls, Ty):
        if Ty is None:
            raise ArgumentError("Option type cannot be none T: " + str(Ty))
        
        if Ty in options:
            return options[Ty]

        ty = to_c_type(Ty)

        class TypedOption(Option):
            T = Ty
            CT = ty
            _Pointer_ = POINTER(ty if not hasattr(ty, '__ty__') else ty.__ty__())

        options[Ty] = TypedOption
        return TypedOption

    @classmethod
    def __from_result__(cls, res, lib):
        inner = read_pointer(cls.CT, res, lib)
        return cls(inner, lib, res)

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

    def __new_resource__ (self, res):
        if isinstance(self.__value__, LibValue):
            self.__value__.__new_resource__(res)
        else:
            self.__resources__.append(res)

    def __free__(self, lib):
        print('free')
        print(self.__resources__)
        print(self.__raw__)

        if self.__raw__ is not None and len(self.__resources__) is not 0:
            lib.free_coption(self.__raw__)
        #for name, res in self.__resources__:
        #    getattr(lib, name)(res)

    def unwrap(self) -> T:
       return self.__value__

types = {}
def submit_type(name, ty):
    types[name] = ty

def get_type(ty):
    if ty in types:
        return types[ty]
    return ty