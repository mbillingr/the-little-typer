def fresh(ctx, name):
    return freshen(set(ctx.keys()), name)


def freshen(used: {str}, name: str) -> str:
    if name in used:
        split = split_name(x)
        return freshen_aux(used, split)
    return name


def freshen_aux(used, split):
    joined = unsplit_name(split)
    if joined in used:
        return freshen_aux(used, next_split_name(split))
    return joined


def next_split_name(split):
    name, num = split
    return name, num + 1


def unsplit_name(split) -> str:
    name, num = split
    return name + number_to_subscript(num)


def number_to_subscript(num: int) -> str:
    s = str(num)
    for digit, subscript in (
        ("0", "₀"),
        ("1", "₁"),
        ("2", "₂"),
        ("3", "₃"),
        ("4", "₄"),
        ("5", "₅"),
        ("6", "₆"),
        ("7", "₇"),
        ("8", "₈"),
        ("9", "₉"),
    ):
        s = s.replace(digit, subscript)
