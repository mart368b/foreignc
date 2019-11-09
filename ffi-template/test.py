from foreignc import *

class ffi_templateLib(BaseLib):
    def __init__(self, src: str):
        super().__init__('ffi_templateLib', src)
        self.ExternName = self.lib.ExternName
        self.ExternName.argtypes = (c_char_p, bool)
        self.ExternName.restypes = lib_char_p('ffi_templateLib')
        self.ExternName = self.lib.ExternName
        self.ExternName.argtypes = (c_char_p, bool)
        self.ExternName.restypes = lib_char_p('ffi_templateLib')

