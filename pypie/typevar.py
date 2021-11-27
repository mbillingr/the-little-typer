from funny_id import hash_id


class TypeVar:
    def __init__(self, typ):
        self.typ = typ

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