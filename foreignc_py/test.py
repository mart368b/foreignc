from foreignc import *
from ctypes import c_void_p, c_char_p, cast

def handler(v):
    print(type(v))
    return c_char_p(v).value

print(__name__)
if __name__ == '__main__':
    class MyLib(BaseLib):
        def __init__(self, src: str):
            super().__init__('MyLib', src)
            self.parse_string = self.lib.parse_string
            self.parse_string.argtypes = (c_char_p,)
            self.parse_string.restype = None

            self.get_string = self.lib.get_string
            self.get_string.restype = lib_char_p('MyLib')
            self.get_string.errcheck = Json

            self.free_string = self.lib.free_string
            self.free_string.argtypes = (c_char_p,)

    lib = MyLib('template_test.dll')

    s = lib.get_string("a")
    lib.parse_string(s)
    print(s)
