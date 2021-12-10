mod alpha;
pub mod basics;
pub mod errors;
mod fresh;
pub mod normalize;
pub mod rep;
mod symbol;
pub mod typechecker;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
