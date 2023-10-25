To setup the environment:

```bash
> pyenv activate pyo3
```

Then look at the first function, `sum_as_string`, and the way it is added to the module.

```bash
> maturin develop
> python
Python 3.11.6 (main, Oct  3 2023, 02:51:45) [Clang 14.0.3 (clang-1403.0.22.14.1)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import pyffi
>>> pyffi.sum_as_string(22, 44)
'66'
```

Now look at `comma_join0`, which takes a `Vec`

```bash
> python
Python 3.11.6 (main, Oct  3 2023, 02:51:45) [Clang 14.0.3 (clang-1403.0.22.14.1)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import pyffi
>>> pyffi.comma_join(["a", "b", "c"])
'a, b, c'
>>> pyffi.comma_join([1, 2, 3])
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: argument 'a': 'int' object cannot be converted to 'PyString'
```


Now look at `comma_join1`, which returns an error

```bash
> python         
Python 3.11.6 (main, Oct  3 2023, 02:51:45) [Clang 14.0.3 (clang-1403.0.22.14.1)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import pyffi
>>> pyffi.comma_join_nonempty([])
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: empty list
```

There are also options for propagating Rust errors.

Now open https://pyo3.rs/v0.20.0/conversions/tables

Now look at `comma_join_py`

Now loko at `make_struct`

```bash
> python
>>> class Foo: pass
... 
>>> o = Foo()
>>> pyffi.make_struct(o)
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
AttributeError: 'Foo' object has no attribute 'my_string'
>>> o.my_string = "hi"
>>> pyffi.make_struct(o)
'hi'
```

```
> python
Python 3.11.6 (main, Oct  3 2023, 02:51:45) [Clang 14.0.3 (clang-1403.0.22.14.1)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import pyffi
>>> pyffi.type_test(2)
'IsInt(2)'
>>> pyffi.type_test("4")
'IsString("4")'
>>> pyffi.type_test([4])
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: argument 'a': failed to extract enum TypeTest ('IsInt | IsString | Point')
- variant IsInt (IsInt): TypeError: failed to extract field TypeTest::IsInt.0, caused by TypeError: 'list' object cannot be interpreted as an integer
- variant IsString (IsString): TypeError: failed to extract field TypeTest::IsString.0, caused by TypeError: 'list' object cannot be converted to 'PyString'
- variant Point (Point): AttributeError: 'list' object has no attribute 'x'
>>> class Point: pass
... 
>>> (p.x, p.y) = (22, 44)
>>> p = Point()
>>> (p.x, p.y) = (22, 44)
>>> pyffi.type_test(p)
'Point { x: 22, y: 44 }'
>>> p.x = "22"
>>> pyffi.type_test(p)
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: argument 'a': failed to extract enum TypeTest ('IsInt | IsString | Point')
- variant IsInt (IsInt): TypeError: failed to extract field TypeTest::IsInt.0, caused by TypeError: 'Point' object cannot be interpreted as an integer
- variant IsString (IsString): TypeError: failed to extract field TypeTest::IsString.0, caused by TypeError: 'Point' object cannot be converted to 'PyString'
- variant Point (Point): TypeError: failed to extract field TypeTest::Point.x, caused by TypeError: 'str' object cannot be interpreted as an integer
```