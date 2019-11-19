from __future__ import annotations
from foreignc import *

class BoxedStruct(Box):
    @staticmethod
    def __free_func__() -> str:
        return 'free_boxed_struct'

    @create_abi('new_boxed_struct', restype='BoxedStruct')
    def new(lib: BaseLib) -> BoxedStruct:
        return lib.__lib__.new_boxed_struct()

    @create_abi('debug_boxed_struct', argtypes=('BoxedStruct',))
    def debug(self) :
        return self.__lib__.debug_boxed_struct(self)

submit_type('BoxedStruct', BoxedStruct)
class JsonStruct(Json):
    @create_abi('new_json_struct', restype='JsonStruct')
    def new(lib: BaseLib) -> JsonStruct:
        return lib.__lib__.new_json_struct()

    @create_abi('debug_json_struct', argtypes=('JsonStruct',))
    def debug(self) :
        return self.__lib__.debug_json_struct(self)

submit_type('JsonStruct', JsonStruct)
class FfiTemplateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__(src)

    @create_abi('does_panic_ffi', restype=LibString)
    def does_panic(self) -> str:
        return self.__lib__.does_panic_ffi()

    @create_abi('get_none_ffi', restype=OPTION(int))
    def get_none(self) -> OPTION(int):
        return self.__lib__.get_none_ffi()

    @create_abi('get_number_ffi', restype=int)
    def get_number(self) -> int:
        return self.__lib__.get_number_ffi()

    @create_abi('get_some_ffi', restype=OPTION(LibString))
    def get_some(self) -> OPTION(str):
        return self.__lib__.get_some_ffi()

    @create_abi('get_string_ffi', restype=LibString)
    def get_string(self) -> str:
        return self.__lib__.get_string_ffi()

    @create_abi('parse_string_ffi', argtypes=(LibString,))
    def parse_string(self, s: str) :
        return self.__lib__.parse_string_ffi(s)

    @create_abi('set_some_ffi', argtypes=(OPTION(LibString),))
    def set_some(self, v: OPTION(str)) :
        return self.__lib__.set_some_ffi(v)
