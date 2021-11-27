class TypeMismatch(Exception):
    """Type checking failed because two types don't match"""


class NotATypeError(Exception):
    """Expected a type but something else was passed"""


class Type:
    """Base class for simple types"""


class ParametricType:
    """Base class for parametric (compound) types"""


def typecheck(obj, typ):
    """Check if obj is a typ.
    If the check succeeds, returns something truthy.
    Otherwise, a TypeError is raised."""
    return typ.check(obj)


def is_type(thing) -> bool:
    """Test if a thing is a type."""
    try:
        if issubclass(thing, Type):
            return True
    except TypeError:
        pass
    if isinstance(thing, ParametricType):
        return True
    return False


def assert_type(thing):
    """Raise NotATypeError if thing is not a type"""
    if not is_type(thing):
        raise NotATypeError(thing)


def are_same(typ) -> bool:
    """Create a function that compares if two objects are the same according to a type"""
    assert_type(typ)
    return typ.compare


def are_same_type(a, b) -> bool:
    """Test if two types are the same"""
    assert_type(a)
    assert_type(b)
    return a == b
