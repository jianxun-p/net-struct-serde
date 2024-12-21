use proc_macro2::token_stream::IntoIter;
use std::iter::Peekable;

pub(crate) fn parse_attr<F>(attrs: &Vec<syn::Attribute>, attr_path: &'static str, mut f: F)
    where F: FnMut(&proc_macro2::TokenStream)
{
    for attr in attrs {
        if let syn::Meta::List(meta_list) = &attr.meta {
            assert!(
                meta_list.path.is_ident(attr_path),
                "Expected \"net_struct\""
            );
            f(&meta_list.tokens);
        }
    }
}

pub(crate) fn expect_ident<I>(it: &mut I, expect_msg: &str) -> String
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    use proc_macro2::TokenTree::*;
    if let Ident(i) = it.next().expect(expect_msg) {
        i.to_string()
    } else {
        panic!("{}", expect_msg);
    }
}

pub(crate) fn expect_group<I>(
    it: &mut I,
    delimiter: proc_macro2::Delimiter,
    expect_msg: &str,
) -> proc_macro2::TokenStream
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    use proc_macro2::TokenTree::*;
    if let Group(g) = it.next().expect(expect_msg) {
        assert_eq!(g.delimiter(), delimiter, "{}", expect_msg);
        g.stream()
    } else {
        panic!("{}", expect_msg);
    }
}

pub(crate) fn consume_punct<I>(it: &mut Peekable<I>, punct: char) -> Option<char>
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    use proc_macro2::TokenTree::*;
    let mut consumed = false;
    if let Some(Punct(p)) = it.peek() {
        consumed = p.as_char() == punct;
    }
    match consumed {
        true => {
            let _ = it.next()?;
            Some(punct)
        }
        false => None,
    }
}

pub(crate) fn consume_ident<I>(it: &mut Peekable<I>) -> Option<String>
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    use proc_macro2::TokenTree::*;
    let mut s = None;
    if let Some(Ident(i)) = it.peek() {
        s = Some(i.to_string());
    }
    if s.is_some() {
        let _ = it.next()?;
    }
    s
}

pub(crate) fn skip_until_punct(it: &mut Peekable<IntoIter>, punct: char) -> Peekable<IntoIter> {
    use proc_macro2::TokenTree::*;
    let new_it = it.skip_while(|tt| match tt {
        Punct(p) => p.as_char() != punct,
        _ => true,
    });
    proc_macro2::TokenStream::from_iter(new_it)
        .into_iter()
        .peekable()
}
