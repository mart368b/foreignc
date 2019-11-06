import json
from ctypes import c_char_p

libs = {}

class BaseLib:
    def __init__(self, name: str, src: str):
        libs[name] = self
        self.f = open(src, 'r').read()

def get_lib(name):
    if name not in libs:
        raise IndexError('No library named: "' + name + '" has been initialized')
    return libs[name]

class Box:
    def __init__(self, ptr, free, lib):
        self._as_parameter_ = ptr
        self.free = free
        self.lib = lib

    def __del__(self):
        if not self.free is None:
            self.free(self.lib, self._as_parameter_)
