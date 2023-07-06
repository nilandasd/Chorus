my_object = {
    foo: 69,
    bar: 80
    inner: {
        qux: 123
    }
}

test = my_object.foo
my_object.bar = 1337

print(test)
print(my_object.bar)