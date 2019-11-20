from api import FfiTemplateLib, JsonStruct, BoxedStruct, Option
from foreignc import ArgumentError
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
    #a = lib.get_some()
    #print(a.unwrap())

    #print(a.unwrap())
    lib.set_some(None)
    lib.set_some('a')
    lib.set_some(Option(None))
    lib.set_some(Option('a'))
    lib.set_some(Option(Option('a')))
    try:
        lib.set_some(Option(Option(None)))
    except ArgumentError as e:
        print('assert_err: "' + str(e) + '"')

