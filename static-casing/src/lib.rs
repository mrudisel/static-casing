pub use static_casing_macros::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        
        assert_eq!("PascalCase", pascal_case!(lit -> lit; "pascal_case"));
        assert_eq!("snake_case", snake_case!(lit -> lit; "SnakeCase"));
        assert_eq!("camelCase", camel_case!(lit -> lit; "camel_case"));
        assert_eq!("camelCase", camel_case!(lit -> lit; "CamelCase"));
        assert_eq!("camelCase", camel_case!(lit -> lit; "camelCase"));
    }    
}