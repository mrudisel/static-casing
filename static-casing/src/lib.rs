pub use static_casing_macros::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        
        assert_eq!("PascalCase", pascal_case!("pascal_case"));
        assert_eq!("snake_case", snake_case!("SnakeCase"));
        assert_eq!("camelCase", camel_case!("camel_case"));
        assert_eq!("camelCase", camel_case!("CamelCase"));
        assert_eq!("camelCase", camel_case!("camelCase"));
    }    
}