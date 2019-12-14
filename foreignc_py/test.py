from api import FfiTemplateLib, JsonStruct, BoxedStruct, Option
from foreignc import ArgumentError
import time

import os

if __name__ == '__main__':

    lib = FfiTemplateLib('template_test.dll')
    #print(lib.get_unknown())
    '''
    print('getting numbers')
    i = 0
    while i < 1000000:
        lib.get_number()
        i += 1
    '''
    print('getting strings')
    #s = BoxedStruct.new(lib)
    s = lib.get_some_number()
    #del s
    print('done')
    print('done')
    print('done')
    print('done')
    print('done')


    #print(lib.get_string())
    #print(lib.parse_string("1234"))

    # Create json object
    #s = JsonStruct.new(lib)
    #s.value = "a"
    #s.debug()
    #print('*'*20)

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
    #lib.set_some(None)
    #lib.set_some('a')
    #lib.set_some(Option(None))
    #lib.set_some(Option('a'))
    #lib.set_some(Option(Option('a')))
    #try:
    #    lib.set_some(Option(Option(None)))
    #except ArgumentError as e:
    #    print('assert_err: "' + str(e) + '"')

