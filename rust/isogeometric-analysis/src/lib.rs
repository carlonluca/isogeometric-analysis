use crate::core::Size;
mod core;

#[cfg(test)]
mod tests {
    use crate::core::Size;
    fn it_works() {
        assert_eq!(Size {
            width: 0,
            height: 0
        }.is_empty(), true);
    }
}
