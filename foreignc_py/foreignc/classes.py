import json, os
from ctypes import c_char_p, cdll, cast, c_wchar_p, c_void_p
from weakref import ref

class BaseLib:
    def __init__(self, src: str):
        self.__lib__ = cdll.LoadLibrary(os.path.abspath(src))

def lib_char_p_to_str(v, r, a):
    return str(v)

class LibValue:
    def __init__(self):
        self.init()

    def init(self):
        self.__lib__ = None
        self.__free__ = None
        self.__value__ = None

    def free_self(self, name: str):
        value = ref(self)
        self.__value__ = property(lambda: value())
        self.__free__ = lambda lib, value: getattr(lib, name)(value.fget())

    def __into__(self):
        return self

    def __errcheck__(cls, v: str, *args, **kwargs):
        return v

    def __del__(self):
        if not hasattr(self, '__free__') or self.__free__ is None:
            return
        self.__free__(self.__lib__, self.__value__)

class LibString(c_char_p, LibValue):
    def __into__(self):
        self.free_self('free_string')
        return self.value.decode('utf-8')

class Box(c_void_p, LibValue):
    def __into__(self):
        self.free_self(self.get_free_func())
        return self

    @staticmethod
    def get_free_func() -> str:
        raise NotImplementedError()

    @property
    def _as_parameter_(self):
        return self.__value__

class Json(LibString, LibValue):
    def __init__(self, lib):
        super(LibValue, self).__init__()
        if lib is not None:
            if hasattr(lib, '__lib__'):
                self.__lib__ = lib.__lib__
            else:
                self.__lib__ = lib
        self.__json__ = ""

    def __repr__ (self):
        return super(LibValue, self).__repr__ ()

    def __into__(self):
        value = LibString.__into__(self)
        return self.wrap_str(value, self.__lib__)

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

    def from_param(self):
        return c_char_p(self.__json__.encode('utf-8'))