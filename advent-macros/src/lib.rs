use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;
use syn::ReturnType;

struct AdventTestFn {
    input: syn::ItemFn,
    simple_literals: Vec<String>,
    full_literals: Vec<String>,
}

impl Parse for AdventTestFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input: syn::ItemFn = input.parse()?;
        let args = &input.sig.inputs;
        if args.len() != 0 {
            return Err(syn::Error::new_spanned(args, "expected no arguments"));
        }
        if input.sig.output != ReturnType::Default {
            return Err(syn::Error::new_spanned(
                input.sig.output,
                "expected no return type",
            ));
        }
        let body = &input.block;
        if body.stmts.len() != 1 {
            return Err(syn::Error::new_spanned(
                body,
                "expected exactly one statement",
            ));
        }

        // stm should be a call to magic() with #[advent_magic] attribute
        let magic = if let syn::Stmt::Semi(expr, _) = &body.stmts[0] {
            expr
        } else {
            return Err(syn::Error::new_spanned(
                &body.stmts[0],
                "expected expression",
            ));
        };

        let magic_call = if let Expr::Call(call) = magic {
            call
        } else {
            return Err(syn::Error::new_spanned(
                magic,
                "expected call to magic(simple, full)",
            ));
        };

        if magic_call.args.len() != 2 {
            return Err(syn::Error::new_spanned(
                &magic_call.args,
                "expected exactly two arguments",
            ));
        }

        // call should be to (simple, full)
        let magic_call_args = &magic_call.args;
        let magic_call_arg0 = if let Expr::Array(arr) = &magic_call_args[0] {
            arr
        } else {
            return Err(syn::Error::new_spanned(
                &magic_call_args[0],
                "expected array argument",
            ));
        };

        if magic_call_arg0.elems.len() != 2 {
            return Err(syn::Error::new_spanned(
                magic_call_arg0,
                "expected exactly two elements",
            ));
        }

        let magic_call_arg0_elem0 = if let Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) = &magic_call_arg0.elems[0]
        {
            lit
        } else {
            return Err(syn::Error::new_spanned(
                &magic_call_arg0.elems[0],
                "expected string literal",
            ));
        };

        let magic_call_arg0_elem1 = if let Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) = &magic_call_arg0.elems[1]
        {
            lit
        } else {
            return Err(syn::Error::new_spanned(
                &magic_call_arg0.elems[1],
                "expected string literal",
            ));
        };

        let magic_call_arg1 = if let Expr::Array(arr) = &magic_call_args[1] {
            arr
        } else {
            return Err(syn::Error::new_spanned(
                &magic_call_args[1],
                "expected array argument",
            ));
        };

        if magic_call_arg1.elems.len() != 2 {
            return Err(syn::Error::new_spanned(
                magic_call_arg1,
                "expected exactly two elements",
            ));
        }

        let magic_call_arg1_elem0 = if let Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) = &magic_call_arg1.elems[0]
        {
            lit
        } else {
            return Err(syn::Error::new_spanned(
                &magic_call_arg1.elems[0],
                "expected string literal",
            ));
        };

        let magic_call_arg1_elem1 = if let Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) = &magic_call_arg1.elems[1]
        {
            lit
        } else {
            return Err(syn::Error::new_spanned(
                &magic_call_arg1.elems[1],
                "expected string literal",
            ));
        };

        let simple_literals = vec![magic_call_arg0_elem0.value(), magic_call_arg0_elem1.value()];
        let full_literals = vec![magic_call_arg1_elem0.value(), magic_call_arg1_elem1.value()];

        Ok(AdventTestFn {
            input,
            simple_literals,
            full_literals,
        })
    }
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
    let AdventTestFn {
        input,
        simple_literals,
        full_literals,
    } = syn::parse_macro_input!(input as AdventTestFn);

    let mut output = proc_macro2::TokenStream::new();

    let day_module = syn::Ident::new(format!("day{day_num:02}").as_str(), Span::call_site());

    let test_name = &input.sig.ident;
    let day_num_str = format!("{:02}", day_num);
    let mut out_fn = quote! {
        #[test]
        fn #test_name() {
            let simple_input = include_str!(concat!("../inputs/", #day_num_str, ".simple.txt"));
            let full_input = include_str!(concat!("../inputs/", #day_num_str, ".full.txt"));

            let simple_expected = [#(#simple_literals),*];
            let full_expected = [#(#full_literals),*];

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
