use crate::symbol::Symbol;
use std::collections::HashSet;

pub fn freshen(used: &HashSet<Symbol>, x: &Symbol) -> Symbol {
    if used.contains(x) {
        let split = split_name(&**x);
        freshen_aux(used, split)
    } else {
        x.clone()
    }
}

fn freshen_aux(used: &HashSet<Symbol>, split: (&str, u32)) -> Symbol {
    let joined = unsplit_name(split);
    if used.contains(joined.as_str()) {
        freshen_aux(used, next_split_name(split))
    } else {
        joined.into()
    }
}

fn split_name(x: &str) -> (&str, u32) {
    let mut n = 0;
    let mut multiplier = 1;

    let mut chi: Vec<_> = x.char_indices().collect();

    let mut name = x;

    loop {
        let (i, ch) = *chi.last().unwrap();
        if ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'].contains(&ch) {
            match ch {
                '₀' => {}
                '₁' => n += 1 * multiplier,
                '₂' => n += 2 * multiplier,
                '₃' => n += 3 * multiplier,
                '₄' => n += 4 * multiplier,
                '₅' => n += 5 * multiplier,
                '₆' => n += 6 * multiplier,
                '₇' => n += 7 * multiplier,
                '₈' => n += 8 * multiplier,
                '₉' => n += 9 * multiplier,
                _ => unreachable!(),
            }
            multiplier *= 10;
            chi.pop();
            name = &x[..i]
        } else {
            return (name, 1 + n);
        }
    }
}

fn unsplit_name((name, num): (&str, u32)) -> String {
    format!("{}{}", name, number_to_subscript(num))
}

fn next_split_name((name, num): (&str, u32)) -> (&str, u32) {
    (name, num + 1)
}

fn number_to_subscript(n: u32) -> String {
    n.to_string()
        .replace('0', "₀")
        .replace('1', "₁")
        .replace('2', "₂")
        .replace('3', "₃")
        .replace('4', "₄")
        .replace('5', "₅")
        .replace('6', "₆")
        .replace('7', "₇")
        .replace('8', "₈")
        .replace('9', "₉")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! set {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_set = HashSet::new();
                $(
                    temp_set.insert(Symbol::new($x));
                )*
                temp_set
            }
        };
    }

    #[test]
    fn freshen_returns_a_clone_of_input_symbol() {
        let s = Symbol::new("A");
        let r = freshen(&set![], &s);
        assert!(s.ptr_eq(&r));
    }

    #[test]
    fn freshen_works() {
        assert_eq!(freshen(&set!["x"], &Symbol::new("x")), "x₁");
        assert_eq!(freshen(&set!["x", "x₁", "x₂"], &Symbol::new("y")), "y");
        assert_eq!(freshen(&set!["x", "x₁", "x₂"], &Symbol::new("x")), "x₃");
        assert_eq!(
            freshen(&set!["r2d", "r2d₀", "r2d₁"], &Symbol::new("r2d")),
            "r2d₂"
        );
        assert_eq!(freshen(&set!["x₁"], &Symbol::new("x₁")), "x₂");
        assert_eq!(freshen(&set![], &Symbol::new("x₁")), "x₁");
        assert_eq!(freshen(&set![], &Symbol::new("₉₉")), "₉₉");
        assert_eq!(freshen(&set!["₉₉"], &Symbol::new("₉₉")), "x₉₉");
        assert_eq!(freshen(&set!["₉₉", "x₉₉"], &Symbol::new("₉₉")), "x₁₀₀");
    }
}
