from foreignc import *


class BoxedStruct(Box):
    @staticmethod
    def get_free_func():
        return 'free_boxed_struct'

    @create_abi('err_boxed_struct', argtypes=(BoxedStruct, int, bool, LibString,), restype=RESULT(int, int))
    def err(self, a: int, b: bool, c: str) -> str:
        return self.__lib__.err(self, a, b, c)

class JsonStruct(Json):
    pass

class FfiTemplateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__(src)

    @create_abi('free_boxed_struct', argtypes=(free_boxed_struct,))
    def free_boxed_struct(self, ptr: free_boxed_struct) -> str:
        return self.__lib__.free_boxed_struct(ptr)

    @create_abi('free_string', argtypes=(LibString,))
    def free_string(self, ptr: str) -> str:
        return self.__lib__.free_string(ptr)
