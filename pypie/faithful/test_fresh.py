from .fresh import freshen


def test_freshen():
    assert freshen(["x"], "x") == "x₁"
    assert freshen(["x", "x₁", "x₂"], "x") == "x₃"
    assert freshen(["x", "x₁", "x₂"], "y") == "y"
    assert freshen(["r2d", "r2d₀", "r2d₁"], "r2d") == "r2d₂"
    assert freshen([], "A") == "A"
    assert freshen(["x₁"], "x₁") == "x₂"
    assert freshen([], "x₁") == "x₁"
    assert freshen([], "₉₉") == "₉₉"
    assert freshen(["₉₉"], "₉₉") == "x₉₉"
    assert freshen(["₉₉", "x₉₉"], "₉₉") == "x₁₀₀"

