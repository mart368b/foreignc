from api import FfiTemplateLib

if __name__ == '__main__':


    lib = FfiTemplateLib('template_test.dll')

    lib.does_panic()

    #print(lib.get_string())
    #print(lib.get_number())

    # Create json object
    #s = lib.get_json_struct()
    #lib.debug_json(s)
    #print(s.str_value)
    # object dropped
    #del s

    # Create box
    #b = lib.get_boxed_struct()
    #print(b)
    # box dropped
    #del b

    #lib.parse_string("a")
    #a = lib.get_some()
    #lib.set_some(a)
