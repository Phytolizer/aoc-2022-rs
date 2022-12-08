use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::token::Bracket;
use syn::token::Fn;
use syn::token::Paren;
use syn::token::Pound;
use syn::AttrStyle;
use syn::Attribute;
use syn::Block;
use syn::Ident;
use syn::ItemFn;
use syn::Path;
use syn::PathArguments;
use syn::PathSegment;
use syn::ReturnType;

struct AdventTestFn {
    input: syn::ItemFn,
}

impl Parse for AdventTestFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input: syn::ItemFn = input.parse()?;
        let args = &input.sig.inputs;
        if args.len() != 2 {
            return Err(syn::Error::new_spanned(args, "expected two arguments"));
        }
        let arg_types = args
            .iter()
            .map(|arg| match arg {
                syn::FnArg::Typed(t) => Ok(&t.ty),
                syn::FnArg::Receiver(_) => {
                    return Err(syn::Error::new_spanned(arg, "expected argument"))
                }
            })
            .collect::<syn::Result<Vec<_>>>()?;
        if arg_types[0].to_token_stream().to_string() != "& str" {
            return Err(syn::Error::new_spanned(arg_types[0], "expected &str"));
        }
        if arg_types[1].to_token_stream().to_string() != "usize" {
            return Err(syn::Error::new_spanned(arg_types[1], "expected usize"));
        }
        if input.sig.output != ReturnType::Default {
            return Err(syn::Error::new_spanned(
                input.sig.output,
                "expected no return type",
            ));
        }
        let body = &input.block;
        if body.stmts.len() != 3 {
            return Err(syn::Error::new_spanned(
                body,
                "expected exactly three statements",
            ));
        }
        let locals = &body.stmts[0..=1];
        for stm in locals {
            let local = if let syn::Stmt::Local(local) = stm {
                local
            } else {
                return Err(syn::Error::new_spanned(stm, "expected let statement"));
            };

            if !local.attrs.is_empty() {
                return Err(syn::Error::new_spanned(
                    &local.attrs[0],
                    "expected no attributes",
                ));
            }
            if local.init.is_none() {
                return Err(syn::Error::new_spanned(
                    local,
                    "expected let statement with initializer",
                ));
            }

            let name = if let syn::Pat::Ident(ident) = &local.pat {
                ident
            } else {
                return Err(syn::Error::new_spanned(
                    &local.pat,
                    "expected let statement with identifier",
                ));
            };

            if !(name.ident == "simple" || name.ident == "full") {
                return Err(syn::Error::new_spanned(
                    name,
                    "expected name to be 'simple' or 'full'",
                ));
            }

            let init = &local.init.as_ref().unwrap().1;
            // init should be &[&str; 2]
            let array = if let syn::Expr::Array(array) = init.as_ref() {
                array
            } else {
                return Err(syn::Error::new_spanned(
                    init,
                    "expected array of 2 string literals",
                ));
            };

            if array.elems.len() != 2 {
                return Err(syn::Error::new_spanned(
                    array,
                    "expected exactly two string literals",
                ));
            }

            for elem in &array.elems {
                let lit = if let syn::Expr::Lit(lit) = elem {
                    lit
                } else {
                    return Err(syn::Error::new_spanned(elem, "expected string literal"));
                };

                if let syn::Lit::Str(_) = lit.lit {
                } else {
                    return Err(syn::Error::new_spanned(lit, "expected string literal"));
                }
            }
        }
        // last stm should be a call to magic() with #[advent_magic] attribute
        let magic = if let syn::Stmt::Expr(expr) = &body.stmts[2] {
            expr
        } else {
            return Err(syn::Error::new_spanned(
                &body.stmts[2],
                "expected expression",
            ));
        };

        let magic_call = if let syn::Expr::Call(call) = magic {
            call
        } else {
            return Err(syn::Error::new_spanned(
                magic,
                "expected call to magic(simple, full)",
            ));
        };

        if magic_call.attrs.len() != 1 {
            return Err(syn::Error::new_spanned(
                magic_call,
                "expected exactly one attribute",
            ));
        }

        let magic_attr = &magic_call.attrs[0];

        if magic_attr.path.segments.len() != 1 {
            return Err(syn::Error::new_spanned(
                &magic_attr.path,
                "expected attribute named advent_magic",
            ));
        }

        let magic_attr_seg = &magic_attr.path.segments[0];

        if magic_attr_seg.ident != "advent_magic" {
            return Err(syn::Error::new_spanned(
                magic_attr_seg,
                "expected attribute named advent_magic",
            ));
        }

        if magic_attr_seg.arguments != syn::PathArguments::None {
            return Err(syn::Error::new_spanned(
                magic_attr_seg,
                "expected attribute with no arguments",
            ));
        }

        if magic_call.args.len() != 2 {
            return Err(syn::Error::new_spanned(
                &magic_call.args,
                "expected exactly two arguments",
            ));
        }

        // call should be to (simple, full)
        let magic_call_args = &magic_call.args;
        let magic_call_arg0 = if let syn::Expr::Path(path) = &magic_call_args[0] {
            path
        } else {
            return Err(syn::Error::new_spanned(
                &magic_call_args[0],
                "expected 'simple' argument",
            ));
        };

        if magic_call_arg0.path.segments.len() != 1 {
            return Err(syn::Error::new_spanned(
                &magic_call_arg0.path,
                "expected 'simple' argument",
            ));
        }

        let magic_call_arg0_seg = &magic_call_arg0.path.segments[0];

        if magic_call_arg0_seg.ident != "simple" {
            return Err(syn::Error::new_spanned(
                magic_call_arg0_seg,
                "expected 'simple' argument",
            ));
        }

        if magic_call_arg0_seg.arguments != syn::PathArguments::None {
            return Err(syn::Error::new_spanned(
                magic_call_arg0_seg,
                "expected 'simple' argument",
            ));
        }

        let magic_call_arg1 = if let syn::Expr::Path(path) = &magic_call_args[1] {
            path
        } else {
            return Err(syn::Error::new_spanned(
                &magic_call_args[1],
                "expected 'full' argument",
            ));
        };

        if magic_call_arg1.path.segments.len() != 1 {
            return Err(syn::Error::new_spanned(
                &magic_call_arg1.path,
                "expected 'full' argument",
            ));
        }

        let magic_call_arg1_seg = &magic_call_arg1.path.segments[0];

        if magic_call_arg1_seg.ident != "full" {
            return Err(syn::Error::new_spanned(
                magic_call_arg1_seg,
                "expected 'full' argument",
            ));
        }

        if magic_call_arg1_seg.arguments != syn::PathArguments::None {
            return Err(syn::Error::new_spanned(
                magic_call_arg1_seg,
                "expected 'full' argument",
            ));
        }

        Ok(AdventTestFn { input })
    }
}

fn get_expecteds(input: &syn::ItemFn) -> (Vec<String>, Vec<String>) {
    let body = &input.block;

    // first stm should be a let binding
    let let_binding = if let syn::Stmt::Local(local) = &body.stmts[0] {
        local
    } else {
        unreachable!()
    };

    let simple = if let syn::Expr::Array(array) = let_binding.init.as_ref().unwrap().1.as_ref() {
        array
    } else {
        unreachable!()
    };

    let let_binding = if let syn::Stmt::Local(local) = &body.stmts[1] {
        local
    } else {
        unreachable!()
    };

    let full = if let syn::Expr::Array(array) = let_binding.init.as_ref().unwrap().1.as_ref() {
        array
    } else {
        unreachable!()
    };

    let unparse = |e: &syn::Expr| -> String {
        let lit = if let syn::Expr::Lit(lit) = e {
            lit
        } else {
            unreachable!()
        };

        let s = if let syn::Lit::Str(s) = &lit.lit {
            s
        } else {
            unreachable!()
        };

        s.value()
    };

    let unparsed_simple = simple.elems.iter().map(unparse).collect::<Vec<_>>();
    let unparsed_full = full.elems.iter().map(unparse).collect::<Vec<_>>();

    (unparsed_simple, unparsed_full)
}

#[proc_macro_attribute]
pub fn advent_test(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if args.is_empty() {
        panic!("expected day number as attribute argument");
    }
    let day_num: usize = syn::parse_macro_input!(args as syn::LitInt)
        .base10_parse()
        .unwrap();
    let AdventTestFn { input } = syn::parse_macro_input!(input as AdventTestFn);

    let mut output = proc_macro2::TokenStream::new();

    let (simple, full) = get_expecteds(&input);

    let day_module = syn::Ident::new(format!("day{day_num:02}").as_str(), Span::call_site());

    let test_name = &input.sig.ident;
    let day_num_str = format!("{:02}", day_num);
    let mut out_fn = quote! {
        #[test]
        fn #test_name() {
            let simple_input = include_str!(concat!("../inputs/", #day_num_str, ".simple.txt"));
            let full_input = include_str!(concat!("../inputs/", #day_num_str, ".full.txt"));

            let simple_expected = [#(#simple),*];
            let full_expected = [#(#full),*];

            for part in 1..=2 {
                for input in [simple_input, full_input] {
                    let expected = if input == simple_input {
                        simple_expected[part - 1]
                    } else {
                        full_expected[part - 1]
                    };

                    let output = crate::#day_module::run(input, part).unwrap();

                    assert_eq!(output, expected);
                }
            }
        }
    };

    out_fn.into_token_stream().into()
}
