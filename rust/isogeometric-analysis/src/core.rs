pub mod size;

#[cfg(test)]
mod tests {
    pub use crate::core::size;

    #[test]
    fn it_works() {
        assert_eq!(size::Size {
            width: 0,
            height: 0
        }.is_empty(), true);
    }
}