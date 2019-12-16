from __future__ import annotations
from foreignc import *

class BoxedCounter(Box):
    @staticmethod
    def __free_func__() -> str:
        return 'free_boxed_counter'

    @create_abi('inc_boxed_counter', argtypes=('BoxedCounter',))
    def inc(self) :
        return self.__lib__.inc_boxed_counter(self).consume()

    @create_abi('new_boxed_counter', restype='BoxedCounter')
    def new(lib: BaseLib) -> BoxedCounter:
        return lib.__lib__.new_boxed_counter().consume()

    @create_abi('inc_boxed_counter', argtypes=('BoxedCounter',))
    def inc(self) :
        return self.__lib__.inc_boxed_counter(self).consume()

    @create_abi('display_boxed_counter', argtypes=('BoxedCounter',))
    def display(self) :
        return self.__lib__.display_boxed_counter(self).consume()

submit_type('BoxedCounter', BoxedCounter)
class JsonCounter(Json):
    pass
submit_type('JsonCounter', JsonCounter)


class FfiTemplateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__(src)

    @create_abi('hello_world_ffi')
    def hello_world(self) :
        return self.__lib__.hello_world_ffi().consume()
