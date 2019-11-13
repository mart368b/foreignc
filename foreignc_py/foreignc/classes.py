import json, os, sys
from ctypes import *
import inspect as ins

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
        #print('Dropping: ' + str(self))
        if hasattr(self, '__lib__') and self.__lib__ is not None:
            self.__free__(self.__lib__)
            #print('Dropped: ' + str(self))

    def __validate__(self):
        return None

class LibString(c_char_p, LibValue):
    _type_ = c_char_p._type_

    def __into__(self, lib):
        super().__into__(lib)
        bytes = self.value
        if bytes is not None:
            return bytes.decode('utf-8')
        else:
            return None

    @classmethod
    def wrap_str(cls, value: str):
        s = cls()
        s.value = value.encode('utf-8')
        return s

    def __free__(self, lib):
        if self:
            lib.free_string(self)
        super().__free__(lib)

    def from_param(v):
        if isinstance(v, str):
            return LibString.wrap_str(v)
        if isinstance(v, LibString):
            return v
        raise ArgumentError('Wrong argument expected a string got ' + str(type(v)))

class Box(c_void_p, LibValue):
    _type_ = c_void_p._type_

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

    def from_param(v):
        print(v)
        print(isinstance(v, Box))
        if isinstance(v, Box):
            return v
        raise ArgumentError('Expected box ' + str(type(v)))

class Json(LibString, LibValue):
    def __init__(self, lib):
        super(LibValue, self).__init__()
        if lib is not None:
            self.__lib__ = lib
        self.__json__ = ""

    def __repr__ (self):
        return super(LibValue, self).__repr__ ()

    def __free__(self, lib):
        LibString.__free__(self, lib)

    def __into__(self, lib):
        super().__into__(lib)
        self.str_value = LibString.__into__(self, lib)
        return self

    @classmethod
    def wrap_str(cls, value: str, lib):
        s = cls(lib)
        s.str_value = value
        s.value = value.encode('utf-8')
        return s

    @classmethod
    def wrap_obj(cls, obj, lib):
        s = cls(lib)
        s.object = obj
        s.value = s.str_value.encode('utf-8')
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

    def from_param(v):
        if isinstance(v, str):
            return Json.wrap_str(v, None)
        if isinstance(v, LibString):
            return Json.wrap_str(v.str_value, None)
        if isinstance(v, Json):
            return v
        return Json.wrap_obj(v, None)

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

options = {}
def OPTION(tt):
    if tt in options:
        return options[tt]

    ty = to_c_type(tt)
    class Option(POINTER(ty), LibValue):
        _type_ = ty

        def __init__(self, v):
            super().__init__()
            if hasattr(ty, 'from_param'):
                self.contents = ty.from_param(v)
            else:
                self.contents = v

        def __free__(self, lib):
            if isinstance(self.value, LibValue):
                self.value.__free__(lib)

        def __into__(self, lib):
            super().__into__(lib)
            if self:
                content = self.contents
                if isinstance(content, LibValue):
                    self.value = self.contents.__into__(lib)
                else:
                    self.value = content.value if hasattr(content, 'value') else content
                return self
            else:
                self.value = None
                return self

        def from_param(v):
            if isinstance(v, Option):
                return Option(v.value)
            if v is None:
                return Option(None)
            if isinstance(v, ty):
                return Option(v)
            if hasattr(ty, 'from_param'):
                return Option(v)
            raise FFiError('Expected ' + str(ty) + ' but got ' + str(type(v)))

        def unwrap(self):
           return self.value

    options[tt] = Option
    return Option