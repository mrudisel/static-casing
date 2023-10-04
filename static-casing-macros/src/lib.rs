

use proc_macro::{TokenStream, TokenTree, Literal, Span, Ident};

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
    let result = match tokens.into_iter().next() {
        Some(TokenTree::Literal(lit)) => handle_lit_str(lit, case),
        Some(TokenTree::Ident(ident)) => handle_ident(ident, case),
        Some(_) => return build_error("expected a string literal or identifier"),
        None => return build_error("expected an input"), 
    };
    
    match result {
        Ok(tree) => TokenStream::from(tree),
        Err(msg) => return build_error(msg),
    }
}

fn handle_lit_str(lit: Literal, case: Case) -> Result<TokenTree, &'static str> {
    let lit_str = lit.to_string(); 
    let mut chars = lit_str.chars();
    
    // strip off the first and last character, which we expect to be quotes
    let first = chars.next();
    let last = chars.by_ref().rev().next(); 
        
    match (first, last) {
        (Some('\"'), Some('\"')) => (),
        (Some('\r'), _) => return Err("raw literals not yet supported"),
        (Some(_), _) | (_, Some(_)) => return Err("expected a string literal"),
        (None, None) => unreachable!("literal with no data?"),
    }
    
    // the actual string should be all thats left after stripping the quotes
    let output = chars.as_str().to_case(case);
    
    Ok(TokenTree::from(Literal::string(&output)))
}


fn handle_ident(ident: Ident, case: Case) -> Result<TokenTree, &'static str> {
    let ident_str = ident.to_string();
    if ident_str.starts_with("r#") {
        return Err("raw identifiers not supported");
    }
    
    let new_ident_str = ident_str.to_case(case);
    
    Ok(TokenTree::from(Ident::new(&new_ident_str, Span::call_site())))
}


fn build_error(message: &str) -> TokenStream {
    // using the Debug impl for str adds quotes around the string 
    let expr = format!("compile_error!({message:?})");
    expr.parse().expect("should be valid tokens")
}