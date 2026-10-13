use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, Ident, ItemEnum, Lit, Path, Token, Type, parse::Parse, parse_quote};

struct MacroArgs {
    generator: Path,
    out_type: Type,
    start: Expr,
    step: Expr,
}

impl Parse for MacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let generator = input.parse()?;

        let mut out_type: Type      = syn::parse_quote! { usize };
        let mut start: Expr         = syn::parse_quote! { 0usize };
        let mut step: Expr          = syn::parse_quote! { 1usize };

        while !input.is_empty() {
            let _: Token![,] = input.parse()?;
            let key: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let key_string = key.to_string();

            match key_string.as_str() {
                "out_type"      => out_type = input.parse()?,
                "start"         => start = input.parse()?,
                "step"          => step = input.parse()?,
                s         => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(" [{}] is not an accepted argument", s)
                    ))
                }
            }
        }


        Ok(Self {
            generator,
            out_type,
            start,
            step,
        })
    }
}


fn eval_expr_to_usize(expr: &Expr) -> Option<usize> {
    let Expr::Lit(expr_lit) = expr else {
        return None;
    };

    let Lit::Int(lit_int) = &expr_lit.lit else {
        return None;
    };

    lit_int.base10_parse::<usize>().ok()
}


struct ConfiguredEnum(ItemEnum);

impl ConfiguredEnum {
    pub fn apply_args(&mut self, args: MacroArgs) -> syn::Result<()> {
        let generator = args.generator;

        let mut curr_idx = eval_expr_to_usize(&args.start)
            .ok_or_else(|| syn::Error::new_spanned(&args.start, "Failed to parse start to usize"))?;
        let step = eval_expr_to_usize(&args.step)
            .ok_or_else(|| syn::Error::new_spanned(&args.start, "Failed to parse step to usize"))?;

        
        for v in &mut self.0.variants {
            v.discriminant = Some((
                Token![=](v.ident.span()),
                parse_quote! { #generator(#curr_idx) }
            ));

            curr_idx = curr_idx.checked_add(step)
                .ok_or_else(|| syn::Error::new_spanned(&v.ident, format!("Index for variant {} too large", v.ident)))?;
        }

        let out_type = args.out_type;
        self.0.attrs.push(parse_quote! { #[repr(#out_type)] });

        Ok(())
    }
}

impl Parse for ConfiguredEnum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl ToTokens for ConfiguredEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item = &self.0;
        tokens.extend(quote! { #item });
    }
}




#[proc_macro_attribute]
pub fn enum_gen(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = match syn::parse::<MacroArgs>(args) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into()
    };

    let mut generator_enum = match syn::parse::<ConfiguredEnum>(item) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into()
    };

    if let Err(e) = generator_enum.apply_args(args) {
        return e.into_compile_error().into();
    }

    let expanded = quote! { #generator_enum };
    return expanded.into();
}
