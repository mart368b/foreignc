import time, json, gc, sys
from pprint import pprint
from foreignc import *
from ctypes import *

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

        @create_abi('parse_string', argtypes=(LibString,))
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

        @create_abi('get_none', restype=OPTION(LibString))
        def get_none(self):
            return self.__lib__.get_none()

        @create_abi('get_some', restype=OPTION(str))
        def get_some(self):
            return self.__lib__.get_some()

        @create_abi('set_some', argtypes=(OPTION(str),))
        def set_some(self, v: str):
            return self.__lib__.set_some(v)

    lib = MyLib('template_test.dll')

    #lib.parse_string("a")

    #print(lib.get_string())
    #print(lib.get_number())

    # Create json object
    s = lib.get_json_struct()
    lib.debug_json(s)
    #print(s.str_value)
    # object dropped
    #del s

    # Create box
    b = lib.get_boxed_struct()
    print(b)
    # box dropped
    del b

    lib.parse_string("a")
    a = lib.get_some()
    lib.set_some(a)
