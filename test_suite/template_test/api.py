from __future__ import annotations
from foreignc import *

class BoxedStruct(Box):
    @staticmethod
    def __free_func__() -> str:
        return 'free_boxed_struct'

    @create_abi('new_boxed_struct', restype='BoxedStruct')
    def new(lib: BaseLib) -> BoxedStruct:
        return lib.__lib__.new_boxed_struct().consume()

    @create_abi('debug_boxed_struct', argtypes=('BoxedStruct',))
    def debug(self) :
        return self.__lib__.debug_boxed_struct(self).consume()

submit_type('BoxedStruct', BoxedStruct)
class JsonStruct(Json):
    @create_abi('new_json_struct', restype='JsonStruct')
    def new(lib: BaseLib) -> JsonStruct:
        return lib.__lib__.new_json_struct().consume()

    @create_abi('debug_json_struct', argtypes=('JsonStruct',))
    def debug(self) :
        return self.__lib__.debug_json_struct(self).consume()

submit_type('JsonStruct', JsonStruct)

class UnknownStruct(RawPointer):
    pass


class FfiTemplateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__(src)

    @create_abi('get_err_ffi', restype=Result[LibString, LibString])
    def get_err(self) -> Result[str, str]:
        return self.__lib__.get_err_ffi().consume()

    @create_abi('get_nested_combined_ffi', restype=Option[Option[Result[Option[LibString], LibString]]])
    def get_nested_combined(self) -> Option[Option[Result[Option[str], str]]]:
        return self.__lib__.get_nested_combined_ffi().consume()

    @create_abi('get_nested_ffi', restype=Option[Option[Option[LibString]]])
    def get_nested(self) -> Option[Option[Option[str]]]:
        return self.__lib__.get_nested_ffi().consume()

    @create_abi('get_none_ffi', restype=Option['u64'])
    def get_none(self) -> Option['u64']:
        return self.__lib__.get_none_ffi().consume()

    @create_abi('get_ok_ffi', restype=Result[LibString, LibString])
    def get_ok(self) -> Result[str, str]:
        return self.__lib__.get_ok_ffi().consume()

    @create_abi('get_some_ffi', restype=Option['u64'])
    def get_some(self) -> Option['u64']:
        return self.__lib__.get_some_ffi().consume()

    @create_abi('get_string_ffi', restype=LibString)
    def get_string(self) -> str:
        return self.__lib__.get_string_ffi().consume()

    @create_abi('get_unknown_ffi', restype=UnknownStruct)
    def get_unknown(self) -> UnknownStruct:
        return self.__lib__.get_unknown_ffi().consume()

    @create_abi('set_nested_ffi', argtypes=(Option[Option['u32']],))
    def set_nested(self, v: Option[Option['u32']]) :
        return self.__lib__.set_nested_ffi(v).consume()

    @create_abi('set_option_ffi', argtypes=(Option['u32'],))
    def set_option(self, v: Option['u32']) :
        return self.__lib__.set_option_ffi(v).consume()
