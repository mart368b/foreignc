from api import FfiTemplateLib, JsonStruct, BoxedStruct
from foreignc import OPTION
if __name__ == '__main__':


    lib = FfiTemplateLib('template_test.dll')

    #print(lib.get_string())

    #print(lib.get_string())
    #print(lib.get_number())

    # Create json object
    #s = JsonStruct.new(lib)
    #s.debug()

    # Create box
    #b = BoxedStruct.new(lib)
    #b.debug()
    #del b
    # box dropped
    #del b

    #lib.parse_string("a")
    a = lib.get_none()

    #print(a.unwrap())
    #lib.set_some(a)
