import typing


def freshen(used: typing.Container[str], x: str) -> str:
    if x in used:
        split = split_name(x)
        return freshen_aux(used, split)
    return x


def freshen_aux(used: typing.Container[str], split) -> str:
    joined = unsplit_name(split)
    if joined in used:
        return freshen_aux(used, next_split_name(split))
    return joined


def split_name(x: str):
    idx = len(x)
    while idx > 0 and x[idx - 1] in SUBSCRIPT_DIGITS:
        idx -= 1

    if idx == 0:
        return "x", subscript_string_to_number(x[idx:])

    return x[:idx], 1 + subscript_string_to_number(x[idx:])


def next_split_name(split):
    name, n = split
    return name, n + 1


def unsplit_name(split):
    name, n = split
    return name + number_to_subscript_string(n)


def number_to_subscript_string(n: int) -> str:
    return "".join(SUBSCRIPTS.get(ch, ch) for ch in str(n))


def subscript_string_to_number(s: str) -> int:
    if not s:
        return 0
    return int("".join(SUBSCRIPTS.get(ch, ch) for ch in s))


SUBSCRIPTS = {
    "0": "₀",
    "1": "₁",
    "2": "₂",
    "3": "₃",
    "4": "₄",
    "5": "₅",
    "6": "₆",
    "7": "₇",
    "8": "₈",
    "9": "₉",
    "₀": "0",
    "₁": "1",
    "₂": "2",
    "₃": "3",
    "₄": "4",
    "₅": "5",
    "₆": "6",
    "₇": "7",
    "₈": "8",
    "₉": "9",
}

SUBSCRIPT_DIGITS = "₀₁₂₃₄₅₆₇₈₉"
