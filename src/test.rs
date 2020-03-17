use client;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(client::add(1, 2), "3");
    }

    #[test]
    fn test_author() {
        assert_eq!(client::author(), "Neil");
    }
}
