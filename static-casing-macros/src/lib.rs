

use proc_macro::{TokenStream, TokenTree, Literal, Span, Ident, Spacing};

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







trait SourceType: Sized {
    fn from_tree(tree: TokenTree) -> Result<Self, &'static str>;
    
    fn recase(&self, case: Case) -> Result<String, &'static str>;
}



impl SourceType for Literal {
    fn from_tree(tree: TokenTree) -> Result<Self, &'static str> {
        match tree {
            TokenTree::Literal(l) => Ok(l),
            _ => Err("expected a string literal"),
        }
    }
    
    fn recase(&self, case: Case) -> Result<String, &'static str> {
        let lit_str = self.to_string(); 
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
        Ok(chars.as_str().to_case(case))
    }
}

impl SourceType for Ident {
    fn from_tree(tree: TokenTree) -> Result<Self, &'static str> {
        match tree {
            TokenTree::Ident(ident) => Ok(ident),
            _ => Err("expected a non-raw identifier"),
        }
    }
    
    fn recase(&self, case: Case) -> Result<String, &'static str> {
        let ident_str = self.to_string();
        if ident_str.starts_with("r#") {
            return Err("raw identifiers not supported");
        }
        
        Ok(ident_str.to_case(case))
    }
}
fn do_casing(tokens: TokenStream, case: Case) -> TokenStream {
    fn inner<Src: SourceType, Dst: DstType, I: Iterator<Item = TokenTree>>(token_iter: &mut I, case: Case) -> TokenStream {    
        let src_type = match token_iter.next().map(Src::from_tree) {
            Some(Ok(src)) => src,
            Some(Err(err)) => return build_error(err),
            None => return build_error("expected an input"),
        };
        
        if token_iter.next().is_some() {
            return build_error("expected a single input token");
        }
        
        match src_type.recase(case) {
            Ok(s) => TokenStream::from(Dst::from_string(s).into()),
            Err(err) => build_error(err), 
        }
    }
    
    
    let mut token_iter = tokens.into_iter();
    
    let (src, dst) = match get_types(&mut token_iter) {
        Ok(pair) => pair,
        Err(err) => return build_error(err),
    };
    
    match (src, dst) {
        (Type::Lit, Type::Lit) => inner::<Literal, Literal, _>(&mut token_iter, case),
        (Type::Lit, Type::Ident) => inner::<Literal, Ident, _>(&mut token_iter, case),
        (Type::Ident, Type::Lit) => inner::<Ident, Literal, _>(&mut token_iter, case),
        (Type::Ident, Type::Ident) => inner::<Ident, Ident, _>(&mut token_iter, case),
    } 

}

fn get_types<I: Iterator<Item = TokenTree>>(token_type: &mut I) -> Result<(Type, Type), &'static str> {
    let src_ident = match token_type.next().map(Ident::from_tree) {
        Some(ident_res) => ident_res?,
        None => return Err("expected a type conversion (lit -> ident; etc)"),
    };
    
    let src_type = match src_ident.to_string().trim() {
        "lit" => Type::Lit,
        "ident" => Type::Ident,
        _ => return Err("expected 'lit' or 'ident'"),
    };
    
    fn verify_punct(token: Option<TokenTree>, expected: char, spacing: Spacing) -> Result<(), &'static str> {
        match token {
            Some(TokenTree::Punct(p)) if p.spacing() == spacing && p.as_char() == expected => Ok(()),
            _ => Err("expected a '->' and a target type marker"),
        }
    }
    
    verify_punct(token_type.next(), '-', Spacing::Joint)?;    
    verify_punct(token_type.next(), '>', Spacing::Alone)?;    
    
    let dst_ident = match token_type.next().map(Ident::from_tree) {
        Some(res) => res?,
        None => return Err("expected a dst type for conversion (lit -> ident; etc)"),
    };
    
    let dst_type = match dst_ident.to_string().trim() {
        "lit" => Type::Lit,
        "ident" => Type::Ident,
        _ => return Err("expected 'lit' or 'ident'"),
    };
    
    match token_type.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ';' && p.spacing() == Spacing::Alone => (),
        _ => return Err("expected ';' after the conversion"),
    }
    
    Ok((src_type, dst_type))
}


enum Type {
    Lit,
    Ident,
}


trait DstType: Into<TokenTree> {
    fn from_string(s: String) -> Self;
}

impl DstType for Literal {
    fn from_string(s: String) -> Self {
        Literal::string(&s)
    }
}

impl DstType for Ident {
    fn from_string(s: String) -> Self {
        Ident::new(&s, Span::call_site())
    }
}


fn build_error(message: &str) -> TokenStream {
    // using the Debug impl for str adds quotes around the string 
    let expr = format!("compile_error!({message:?})");
    expr.parse().expect("should be valid tokens")
}