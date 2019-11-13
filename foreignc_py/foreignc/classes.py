import json, os, sys
from ctypes import *
from weakref import ref

class BaseLib:
    def __init__(self, src: str):
        self.__lib__ = cdll.LoadLibrary(os.path.abspath(src))

def lib_char_p_to_str(v, r, a):
    return str(v)

class LibValue:
    def __into__(self, lib):
        self.__lib__ = lib
        return self

    def __free__(self, lib):
        pass

    def __del__(self):
        if hasattr(self, '__lib__') and self.__lib__ is not None:
            self.__free__(self.__lib__)
            print('Dropped: ' + str(self))

    def __validate__(self):
        return None

class LibString(c_char_p, LibValue):
    def __init__(self, v):
        if v is not None:
            super().__init__(v.encode('utf-8'))
        else:
            super().__init__(v)

    def __into__(self, lib):
        super().__into__(lib)
        bytes = self.value
        if bytes is not None:
            return bytes.decode('utf-8')
        else:
            return None

    def __free__(self, lib):
        if self:
            lib.free_string(self)
        super().__free__(lib)

    def from_param(self, **kwargs):
        return LibString(self)

class Box(c_void_p, LibValue):
    def __into__(self, lib):
        super().__into__(lib)
        return self

    def __free__(self, lib):
        getattr(lib, self.get_free_func())(self)
        super().__free__(lib)

    @staticmethod
    def get_free_func() -> str:
        raise NotImplementedError()

    @property
    def _as_parameter_(self):
        return self

class Json(LibString, LibValue):
    def __init__(self, lib):
        super(LibValue, self).__init__()
        if lib is not None:
            if hasattr(lib, '__lib__'):
                self.__lib__ = lib
            else:
                self.__lib__ = lib
        self.__json__ = ""

    def __repr__ (self):
        return super(LibValue, self).__repr__ ()

    def __into__(self, lib):
        return self.wrap_str(LibString.__into__(self, lib), lib)

    @classmethod
    def wrap_str(cls, value: str, lib: BaseLib):
        s = cls(lib)
        s.str_value = value
        return s

    @classmethod
    def wrap_obj(cls, obj, lib: BaseLib):
        s = cls(lib)
        s.object = obj
        return s

    @property
    def str_value(self):
        return self.__json__

    @str_value.setter
    def str_value(self, s: str):
        self.__json__ = s

    @property
    def object(self):
        return json.loads(self.__json__)

    @object.setter
    def object(self, v: str):
        self.__json__ = json.dumps(v)

    def from_param(self, **kwargs):
        return c_char_p(self.__json__.encode('utf-8'))

def convert_ty(ty):
    if isinstance(ty, LibValue):
        return ty
    if ty is int:
        return c_int
    if ty is bool:
        return c_bool
    if ty is float:
        return c_float
    return ty

pointer_types = {}

def as_pointer(key, cls):
    if key in pointer_types:
        return pointer_types[key]
    else:
        cls.ptr = POINTER(cls)
        pointer_types[key] = cls
        return cls

def ARGRESULT(ty):
    class ArgResult(Structure):
        _fields_ = [
            ('inner_value', POINTER(c_int)),
            ('error', LibString)
        ]
    return ArgResult

def OPTION(ty):
    class Option(Structure, LibValue):
        _fields_ = [('inner_value', ty)]
        def __init__(self, v: ty):
            super().__init__()
            if hasattr(ty, 'from_param'):
                self.inner_value = ty.from_param()
            else:
                self.inner_value = v

        def __free__(self, lib):
            print('free')
            if isinstance(self.inner_value, LibValue) and not self.is_unwrapped():
                self.inner_value.__free__(lib)
            super().__free__(lib)

        def is_unwrapped(self):
            return hasattr(self, 'inner_unwrapped')

        def unwrap(self):
            if self.inner_value is None:
                return None
            if self.is_unwrapped():
                return self.inner_unwrapped
            if isinstance(self.inner_value, LibValue):
                self.k = self.inner_value
                v = self.inner_value.__into__(self.__lib__)
                self.inner_unwrapped = v
                return v
            else:
                return self.inner_value

        def __validate__(self):
            if self.is_unwrapped():
                return "Option have already been unwraped " + str(self)
            return None

    return as_pointer(ty, Option)