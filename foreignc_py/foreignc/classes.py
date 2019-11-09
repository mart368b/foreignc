import json, os
from ctypes import c_char_p, cdll, cast

libs = {}

class BaseLib:
    def __init__(self, name: str, src: str):
        libs[name] = self
        self.lib = cdll.LoadLibrary(os.path.abspath(src))

def get_lib(name):
    if name not in libs:
        raise IndexError('No library named: "' + name + '" has been initialized')
    return libs[name]

def lib_char_p_to_str(v, r, a):
    return str(v)

def lib_char_p(name: str):

    class LibStringPointer(c_char_p):
        def __str__(self):
            return self.value.decode('utf-8')

        def __del__(self):
            # Deallocate the string.
            print('Droping')
            get_lib(name).free_string(self)

    return LibStringPointer


class Box:
    def __init__(self, value, free_func, lib_name:str):
        self.__value__ = value
        self.free = free_func
        self.lib = get_lib(lib_name)

    @property
    def _as_parameter_(self):
        return self.__value__

    def __del__(self):
        if not self.free is None:
            self.free(self.lib, self.__value__)

class Json:
    def __init__(self, value, *args, **kwargs):
        if isinstance(value, c_char_p):
            self.__value__ = str(value)
        elif isinstance(value, str):
            self.__value__ = value
        elif isinstance(value, Json):
            self.__value__ = value.__value__
        else:
            self.__value__ = json.dumps(value, *args, **kwargs)

        self.object = property(self.get_object, self.set_object)
        self.string = property(self.get_string, self.set_string)

    def __str__(self):
        return self.get_string()

    def get_string(self):
        return self.__value__

    def set_string(self, v: str):
        self.__value__ = v

    def get_object(self, *args, **kwargs):
        return json.loads(self.__value__, *args, **kwargs)

    def set_object(self, v: str, *args, **kwargs):
        self.__value__ = json.dumps(v, *args, **kwargs)

    @property
    def _as_parameter_(self):
        return c_char_p(self.__value__.encode('utf-8'))