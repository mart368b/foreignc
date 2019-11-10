def deref(r, *args, **kwargs):
    if r:
        return r[0]
    else:
        raise TypeError('Reference is null ' + str(r))