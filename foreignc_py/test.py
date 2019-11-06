from foreignc_py import *

print(__name__)
if __name__ == '__main__':
    class MyLib(BaseLib):
        def __init__(self, src: str):
            super().__init__('MyLib', src)

        def free_string(self, v):
            print('Dropped ' + str(v))

    MyLib('f.txt')

    @use_lib('MyLib')
    @map_arg(lambda v: str(v), mapped=['b'])  # Map a to JsonObject
    @box_res(MyLib.free_string)
    def my_func(a, b, c, lib=None) -> Box:
        print(lib)
        return a

    a = my_func(1, 2, 3)
    del a
    print("a")