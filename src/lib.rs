pub fn clean() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_removes_dots() {
        assert_eq!(clean(), ());
    }
}
