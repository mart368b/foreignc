from __future__ import annotations
from foreignc import *



class FfiTemplateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__(src)

    @create_abi('AExternName', argtypes=(Option[LibString], bool,), restype='u32')
    def ASomeFunc(self, a: Option[str], b: bool) -> 'u32':
        return self.__lib__.AExternName(a, b).consume()

    @create_abi('BExternName', argtypes=(LibString, bool,), restype='u32')
    def BSomeFunc(self, a: str, b: bool) -> 'u32':
        return self.__lib__.BExternName(a, b).consume()
