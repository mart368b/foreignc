from __future__ import annotations
from pydoc import locate
from foreignc import *

class BoxedStruct(Box):
    @staticmethod
    def get_free_func():
        return 'free_boxed_struct'

class JsonStruct(Json):
    pass
class FfiTemplateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__(src)

    @create_abi('does_panic_ffi', restype=LibString)
    def does_panic(self) -> str:
        return self.__lib__.does_panic_ffi()
