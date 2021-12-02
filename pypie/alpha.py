from pypie import Expr


def is_alpha_equivalent(e1: Expr, e2: Expr) -> bool:
    return alpha_eqiv_aux(0, (), (), e1, e2)


def alpha_eqiv_aux(lvl, b1, b2, e1, e2):
    match (e1, e2):
        case (x, y):
            # TODO: check if they are bound variables
            return x == y
        case _: raise NotImplementedError(e1, e2)
