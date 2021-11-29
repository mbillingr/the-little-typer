from funny_id import hash_id


class TypeVar:
    def __init__(self, typ):
        self.typ = typ
        self.attributes = {}

    def request(self, attr):
        try:
            return self.attributes[attr]
        except KeyError:
            value = TypeVar(getattr(self.typ, attr))
            self.attributes[attr] = value
            return value

    def __call__(self, *args):
        return self.typ.call_var(*args)

    def check(self, obj):
        if isinstance(obj, TypeVar):
            obj_type = obj.typ
        else:
            obj_type = type(obj)

        if obj_type != self:
            raise TypeError(self, obj_type)

        return "ok"

    def __repr__(self):
        return hash_id(hash(self))
