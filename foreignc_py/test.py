import time, json, gc, sys
from pprint import pprint
from foreignc import *
from ctypes import c_void_p, c_char_p, cast, c_wchar_p

def handler(v):
    print(type(v))
    return c_char_p(v).value

print(__name__)
if __name__ == '__main__':

    class BoxedStruct(Box):
        @staticmethod
        def get_free_func():
            return 'free_boxed_struct'


    class JsonStruct(Json):
        pass

    class MyLib(BaseLib):
        def __init__(self, src: str):
            super().__init__(src)

        @create_abi('get_string', restype=LibString)
        def get_string(self) -> str:
            return self.__lib__.get_string()

        @create_abi('get_number', restype=int)
        def get_number(self) -> int:
            return self.__lib__.get_number()

        @create_abi('parse_string', argtypes=(c_wchar_p,))
        def parse_string(self, v: str) -> str:
            return self.__lib__.parse_string(v)

        @create_abi('get_boxed_struct', restype=BoxedStruct)
        def get_boxed_struct(self) -> BoxedStruct:
            return self.__lib__.get_boxed_struct()

        @create_abi('debug_box', argtypes=(BoxedStruct,))
        def debug_box(self, b):
            return self.__lib__.debug_box(b)

        @create_abi('get_json_struct', restype = JsonStruct)
        def get_json_struct(self) -> JsonStruct:
            return self.__lib__.get_json_struct()

        @create_abi('debug_json', argtypes=(JsonStruct,))
        def debug_json(self, b):
            return self.__lib__.debug_json(b)

    lib = MyLib('template_test.dll')

    s = lib.get_json_struct()
    pprint(s.object)
    lib.debug_json(s)

    print(sys.getrefcount(s))
    del s

    now = time.time()
    while(time.time() < now + 1000):
        pass
