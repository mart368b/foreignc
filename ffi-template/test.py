from foreignc import *



class JsonStruct(Json):
    pass

class FfiTemplateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__('FfiTemplateLib', src)

        @create_abi('AExternName', argtypes=(OPTION(LibString), bool,), restype=LibString)
            def ASomeFunc(self, a: OPTION(LibString), b: bool) -> str:
                return self.__lib__.ASomeFunc()

        @create_abi('BExternName', argtypes=(LibString, bool,), restype=LibString)
            def BSomeFunc(self, a: str, b: bool) -> str:
                return self.__lib__.BSomeFunc()

        