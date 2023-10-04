

use proc_macro::{TokenStream, TokenTree, Literal, Span};

use convert_case::{Case, Casing};



#[proc_macro]
pub fn camel_case(tokens: TokenStream) -> TokenStream {
    do_casing(tokens, Case::Camel)
}

#[proc_macro]
pub fn snake_case(tokens: TokenStream) -> TokenStream {
    do_casing(tokens, Case::Snake)
}


#[proc_macro]
pub fn pascal_case(tokens: TokenStream) -> TokenStream {
    do_casing(tokens, Case::Pascal)
}


fn do_casing(tokens: TokenStream, case: Case) -> TokenStream {
    let mut token_iter = tokens.into_iter();
    let literal = match token_iter.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string(),
        Some(_) => return build_error("expected a string literal"),
        None => return build_error("expected an input"), 
    };
    
    let mut chars = literal.chars();
    
    // strip off the first and last character, which we expect to be quotes
    let first = chars.next();
    let last = chars.by_ref().rev().next(); 
        
    match (first, last) {
        (Some('\"'), Some('\"')) => (),
        (Some('\r'), _) => return build_error("raw literals not yet supported"),
        (Some(_), _) | (_, Some(_)) => return build_error("expected a string literal"),
        (None, None) => unreachable!("literal with no data?"),
    }
    
    // the actual string should be all thats left after stripping the quotes
    let output = chars.as_str().to_case(case);
    
    let replacement = Literal::string(&output);
    let mut out = TokenStream::new();
    out.extend([TokenTree::from(replacement)]);
    out 
}


fn build_error(message: &str) -> TokenStream {
    // using the Debug impl for str adds quotes around the string 
    let expr = format!("compile_error!({message:?})");
    expr.parse().expect("should be valid tokens")
}