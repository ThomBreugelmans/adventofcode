#[macro_use]p
extern crate lazy_static;
extern crate proc_macro;

use std::collections::{BTreeMap, BTreeSet};
use std::iter;
use std::result::Result as StdResult;
use std::sync::Mutex;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{ParseStream, Parser};
use syn::*;

type DayMap = BTreeMap<String, BTreeSet<String>>;

lazy_static! {
    static ref DAYS: Mutex<Option<DayMap>> = Mutex::new(Some(Default::default()));
}

#[proc_macro_attribute]
pub fn day(
    attr: proc_macro::TokenStream,
    mut item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    assert!(attr.is_empty());

    fn check_impl(impl_: ItemImpl) -> StdResult<(Type, Type), Box<dyn ToTokens>> {
        use self::GenericArgument::Type as GaType;

        let mut trait_ = match impl_.trait_ {
            Some(trait_) => trait_,
            _ => return Err(Box::new(impl_)),
        };

        let PathSegment { ident, arguments } = match trait_.1.segments.last() {
            Some(punctuated::Pair::End(_)) => trait_.1.segments.pop().unwrap().into_value(),
            _ => return Err(Box::new(trait_.1)),
        };

        if ident.to_string() != "Dependency" {
            return Err(Box::new(ident));
        }
        let genargs = match arguments {
            PathArguments::AngleBracketed(genargs) => genargs,
            _ => return Err(Box::new(arguments)),
        };

        let typearg = match genargs
            .args
            .iter()
            .filter(|ga| match *ga {
                GaType(_) => true,
                _ => false,
            })
            .count()
        {
            1 => genargs
                .args
                .into_iter()
                .filter_map(|ga| match ga {
                    GaType(ty) => Some(ty),
                    _ => None,
                })
                .next()
                .unwrap(),
            _ => return Err(Box::new(genargs)),
        };

        Ok((typearg, *impl_.self_ty))
    }

    let item2 = item.clone();
    let impl_ = parse_macro_input!(item2 as ItemImpl);
    let (dependency, dependent) = match check_impl(impl_) {
        Ok(v) => v,
        Err(e) => {
            return Error::new_spanned(e, "expected `impl Dependency<...> for ...`")
                .to_compile_error()
                .into()
        }
    };

    item.extend(iter::once(proc_macro::TokenStream::from(
        (quote! {
            impl __missing_dependency_attribute__<#dependency> for #dependent {}
        })
        .into_token_stream(),
    )));

    let dependency = dependency.into_token_stream().to_string();
    let dependent = dependent.into_token_stream().to_string();

    let mut panic = None;
    if let Some(map) = DAYS.lock().unwrap().as_mut() {
        if !map
            .entry(dependency.clone())
            .or_default()
            .insert(dependent.clone())
        {
            panic = Some(format!(
                "Duplicate dependency: {} on {}",
                dependent, dependency
            ))
        }
    } else {
        panic = Some("Adding dependencies after `define_dependencies!` invocation".into());
    }
    if let Some(msg) = panic {
        panic!(msg);
    }

    item
}

#[proc_macro]
pub fn define_days(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (path1, ty1, path2, ty2) = match (|stream: ParseStream| {
        let path1: TypePath = stream.parse()?;
        let _: Token![,] = stream.parse()?;
        let ty1: Type = stream.parse()?;
        let _: Token![,] = stream.parse()?;
        let path2: TypePath = stream.parse()?;
        let _: Token![,] = stream.parse()?;
        let ty2: Type = stream.parse()?;
        Ok((path1, ty1, path2, ty2))
    })
    .parse(input)
    {
        Err(e) => return e.to_compile_error().into(),
        Ok(v) => v,
    };

    let map = DAYS.lock().unwrap().take();
    // drop DEPS lock
    if let Some(map) = map {
        let typeid = &quote!(::std::any::TypeId);
        let path1 = &path1;
        let path2 = &path2;
        let elems = map.iter().flat_map(|(k, vs)| {
            vs.iter().map(move |v| {
                let k: TokenStream = k.parse().unwrap();
                let v: TokenStream = v.parse().unwrap();
                quote!((#typeid::of::<#k>, #path1::<#k>, #typeid::of::<#v>, #path1::<#v>, #path2::<#k, #v>))
            })
        });

        quote!(
            const AOCDAYS: &[(
                fn() -> #typeid,
                #ty1,
                fn() -> #typeid,
                #ty1,
                #ty2
            )] = &[#(#elems),*];
        )
        .into()
    } else {
        panic!("`define_dependencies!` invoked twice");
    }
}
