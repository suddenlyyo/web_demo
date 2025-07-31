use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, Data, DeriveInput, Expr, ExprLit, Fields, GenericArgument, Ident, Lit, Meta, NestedMeta, Path, PathArguments, Type, parse_macro_input, spanned::Spanned};

/// 检查类型是否是 Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                return true;
            }
        }
    }
    false
}

/// 提取 Option 的内部类型
fn extract_option_inner(ty: &Type) -> Option<Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty.clone());
                    }
                }
            }
        }
    }
    None
}

/// 检查类型是否实现了 Validatable trait
fn is_validatable_type(ty: &Type) -> bool {
    // 在实际实现中，这里需要更复杂的检查
    // 简化版：检查类型是否是自定义类型（不是基本类型）
    match ty {
        Type::Path(type_path) => {
            if type_path.path.segments.len() == 1 {
                let ident = type_path.path.segments[0].ident.to_string();
                !["String", "str", "i32", "i64", "u32", "u64", "f32", "f64", "bool", "char"].contains(&ident.as_str())
            } else {
                true
            }
        },
        _ => false,
    }
}

/// 实现 Validatable trait 的派生宏
#[proc_macro_derive(Validatable, attributes(validate))]
pub fn derive_validatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => {
                return syn::Error::new(input.span(), "只支持包含命名字段的结构体")
                    .to_compile_error()
                    .into();
            },
        },
        Data::Enum(data) => {
            // 枚举处理逻辑
            let variants = data.variants;
            let variant_validations = variants.iter().map(|variant| {
                let variant_name = &variant.ident;

                // 修复：统一返回 Punctuated<Field> 类型
                let fields = match &variant.fields {
                    Fields::Named(fields) => &fields.named,
                    Fields::Unnamed(fields) => {
                        return syn::Error::new(fields.span(), "枚举变体必须使用命名字段").to_compile_error();
                    },
                    Fields::Unit => &Punctuated::new(), // 返回空字段列表
                };

                let field_validations = fields.iter().filter_map(|f| {
                    // 与结构体字段相同的处理逻辑
                    // 这里简化处理，实际需要完整实现
                    Some(quote! {
                        // 枚举变体字段验证
                    })
                });

                quote! {
                    #struct_name::#variant_name { .. } => {
                        #(#field_validations)*
                    }
                }
            });

            let expanded = quote! {
                impl #generics Validatable for #struct_name #generics {
                    fn validate(&self) -> Result<(), ValidationErrorEnum> {
                        match self {
                            #(#variant_validations)*
                        }
                        Ok(())
                    }
                }
            };

            return TokenStream::from(expanded);
        },
        _ => {
            return syn::Error::new(input.span(), "只支持结构体和枚举")
                .to_compile_error()
                .into();
        },
    };

    // 为每个字段生成验证代码
    let field_validations = fields.iter().filter_map(|f| {
        let field_name = f.ident.as_ref()?;
        let field_ty = &f.ty;
        let field_ident_str = field_name.to_string();

        // 查找 validate 属性
        let validate_attr = f
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("validate"))?;

        // 解析属性参数
        let mut desc = field_ident_str.clone();
        let mut rules = Vec::new();
        let mut length = None;
        let mut date_format = None;
        let mut number_min = None;
        let mut number_max = None;

        // 修复：直接访问 attr.meta 而不是使用 parse_meta()
        let meta = &validate_attr.meta;

        // 修复：使用新的方式解析 MetaList
        if let Meta::List(list) = meta {
            let parser = Punctuated::<Meta, Comma>::parse_terminated;
            let nested = match parser.parse2(list.tokens.clone()) {
                Ok(n) => n,
                Err(_) => return None,
            };

            for meta in nested {
                match meta {
                    Meta::Path(path) => {
                        if let Some(rule) = path.get_ident() {
                            let rule_str = rule.to_string();
                            match rule_str.as_str() {
                                "NotNone" => rules.push(quote! { ValidationRulesEnum::NotNone }),
                                "Length" => rules.push(quote! { ValidationRulesEnum::Length }),
                                "ExistLength" => rules.push(quote! { ValidationRulesEnum::ExistLength }),
                                "Date" => rules.push(quote! { ValidationRulesEnum::Date }),
                                "Time" => rules.push(quote! { ValidationRulesEnum::Time }),
                                "DateTime" => rules.push(quote! { ValidationRulesEnum::DateTime }),
                                "NumberMin" => rules.push(quote! { ValidationRulesEnum::NumberMin }),
                                "NumberMax" => rules.push(quote! { ValidationRulesEnum::NumberMax }),
                                "Structure" => rules.push(quote! { ValidationRulesEnum::Structure }),
                                _ => {},
                            }
                        }
                    },
                    Meta::NameValue(nv) => {
                        let key = nv.path.get_ident()?.to_string();
                        match key.as_str() {
                            "desc" => {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
                                    desc = s.value();
                                }
                            },
                            "length" => {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
                                    length = Some(s.value());
                                }
                            },
                            "date_format" => {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
                                    let fmt = match s.value().as_str() {
                                        "Time" => quote! { DateTimeFormatEnum::Time },
                                        "DateTime" => quote! { DateTimeFormatEnum::DateTime },
                                        "DateTimeStr" => quote! { DateTimeFormatEnum::DateTimeStr },
                                        "Year" => quote! { DateTimeFormatEnum::Year },
                                        "YearNoSplit" => quote! { DateTimeFormatEnum::YearNoSplit },
                                        "DateTimePattern" => {
                                            quote! { DateTimeFormatEnum::DateTimePattern }
                                        },
                                        "None" => quote! { DateTimeFormatEnum::None },
                                        _ => return None,
                                    };
                                    date_format = Some(fmt);
                                }
                            },
                            "number_min" => {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) = &nv.value {
                                    number_min = Some(i.base10_parse::<i64>().ok()?);
                                }
                            },
                            "number_max" => {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) = &nv.value {
                                    number_max = Some(i.base10_parse::<i64>().ok()?);
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }
        }

        // 检查是否是嵌套结构体
        let is_nested_structure = rules.iter().any(|r| r.to_string().contains("Structure"));
        let is_option = is_option_type(field_ty);

        // 处理嵌套结构体
        if is_nested_structure {
            let inner_validation = if is_option {
                // 处理 Option<T> 类型
                if let Some(inner_ty) = extract_option_inner(field_ty) {
                    if is_validatable_type(&inner_ty) {
                        quote! {
                            if let Some(inner) = &self.#field_name {
                                inner.validate()?;
                            }
                        }
                    } else {
                        return Some(syn::Error::new(field_ty.span(), "嵌套结构体必须实现 Validatable trait").to_compile_error());
                    }
                } else {
                    return None;
                }
            } else {
                // 处理直接嵌套类型
                if is_validatable_type(field_ty) {
                    quote! {
                        self.#field_name.validate()?;
                    }
                } else {
                    return Some(syn::Error::new(field_ty.span(), "嵌套结构体必须实现 Validatable trait").to_compile_error());
                }
            };

            // 对于嵌套结构体，忽略其他验证规则
            return Some(quote! {
                #inner_validation
            });
        }

        // 处理基本类型验证
        let rule_builder = quote! {
            let mut rule = ValidationRule::new(#desc);
        };

        let rules_builder = rules.iter().map(|rule| {
            quote! {
                rule = rule.with_rule(#rule);
            }
        });

        let length_builder = if let Some(len) = &length {
            quote! {
                rule = rule.with_length(#len);
            }
        } else {
            quote! {}
        };

        let date_format_builder = if let Some(fmt) = &date_format {
            quote! {
                rule = rule.with_date_format(#fmt);
            }
        } else {
            quote! {}
        };

        let number_range_builder = if number_min.is_some() || number_max.is_some() {
            let min = number_min
                .map(|v| quote! { Some(#v) })
                .unwrap_or(quote! { None });
            let max = number_max
                .map(|v| quote! { Some(#v) })
                .unwrap_or(quote! { None });
            quote! {
                rule = rule.with_number_range(#min, #max);
            }
        } else {
            quote! {}
        };

        // 处理 Option 类型
        let value_access = if is_option {
            quote! {
                if let Some(val) = &self.#field_name {
                    val.to_string()
                } else {
                    String::new()
                }
            }
        } else {
            quote! {
                self.#field_name.to_string()
            }
        };

        // 最终字段验证代码
        Some(quote! {
            {
                #rule_builder
                #(#rules_builder)*
                #length_builder
                #date_format_builder
                #number_range_builder
                let value = #value_access;
                ParameterValidator::validate_value(&value, &rule)?;
            }
        })
    });

    // 生成完整的 impl 块
    let expanded = quote! {
        impl #generics Validatable for #struct_name #generics {
            fn validate(&self) -> Result<(), ValidationErrorEnum> {
                #(#field_validations)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
