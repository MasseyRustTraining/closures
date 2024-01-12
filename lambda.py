def inc_f(a):
    a += 1
    return (lambda x: x + a)

f = inc_f(1)
assert f(1) == 3

f = inc_f(5)
assert f(1) == 7
