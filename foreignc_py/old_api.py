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

    @create_abi('get_json_struct', restype=JsonStruct)
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