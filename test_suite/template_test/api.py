from foreignc import *

class ffi_templateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__('ffi_templateLib', src)
        self.get_string = self.lib.get_string
        self.get_string.argtypes = ()
        self.get_string.restypes = lib_char_p('ffi_templateLib')
        
        self.parse_string = self.lib.parse_string
        self.parse_string.argtypes = (c_char_p)
        self.parse_string.restypes = None
        
        self.free_boxed_struct = self.lib.free_boxed_struct
        self.free_boxed_struct.argtypes = (c_void)
        self.free_boxed_struct.restypes = None
        
        self.free_string = self.lib.free_string
        self.free_string.argtypes = (c_char)
        self.free_string.restypes = None
        


class BoxedStruct(Box):
    def __init__(self, value):
        super().__init__(value, ffi_templateLib.free_boxed_struct, 'ffi_templateLib')

class JsonStruct(Json):
    def __init__(self, value):
        super().__init__(value)
