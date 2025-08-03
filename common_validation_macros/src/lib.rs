//! # Common Validation Macros 验证宏库
//!
//! 提供用于自动生成验证代码的派生宏。
//!
//! ## 功能特性
//!
//! - 为结构体自动生成 [Validatable] 实现
//! - 支持多种验证属性（not_null、length、date_format等）
//! - 自动处理嵌套结构体的验证
//! - 支持Option和Vec类型的验证
//!
//! ## 使用示例
//!
//! ```rust
//! use common_validation::{Validatable, DateTimeFormatEnum};
//!
//! #[derive(ValidatableImpl)]
//! struct User {
//!     #[validate(not_null, length_range(min = 3, max = 20), desc = "用户名")]
//!     username: String,
//!     
//!     #[validate(not_null, date_format = Year, desc = "生日")]
//!     birthday: String,
//!     
//!     #[validate(min = 0, max = 150, desc = "年龄")]
//!     age: u8,
//! }
//!
//! let user = User {
//!     username: "test".to_string(),
//!     birthday: "1990-01-01".to_string(),
//!     age: 30,
//! };
//!
//! assert!(user.validate().is_ok());
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, LitInt, LitStr, PathArguments, Type, parse_macro_input};

/// 检查类型是否是指定的类型名
///
/// # 参数
///
/// * `ty` - 要检查的类型
/// * `type_name` - 目标类型名称
///
/// # 返回值
///
/// 如果类型匹配返回true，否则返回false
fn is_type_of(ty: &Type, type_name: &str) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == type_name;
        }
    }
    false
}

/// 检查类型是否为数字类型
///
/// # 参数
///
/// * `ty` - 要检查的类型
///
/// # 返回值
///
/// 如果是数字类型返回true，否则返回false
fn is_number_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = segment.ident.to_string();
            return ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64"].contains(&ident.as_str());
        }
    }
    false
}

/// 检查类型是否为字符串类型
///
/// # 参数
///
/// * `ty` - 要检查的类型
///
/// # 返回值
///
/// 如果是字符串类型返回true，否则返回false
fn is_string_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let ident = segment.ident.to_string();
                ident == "String" || ident == "str"
            } else {
                false
            }
        },
        _ => false,
    }
}

/// 获取 Option<T> 或 Vec<T> 的内部类型
///
/// # 参数
///
/// * `ty` - 要提取内部类型的容器类型
///
/// # 返回值
///
/// 如果是容器类型返回内部类型，否则返回None
fn extract_inner_type(ty: &Type) -> Option<Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    return Some(inner_ty.clone());
                }
            }
        }
    }
    None
}

/// 实现 Validatable trait 的派生宏
///
/// 为结构体自动生成 [Validatable] trait 的实现，支持各种验证属性。
///
/// # 支持的属性
///
/// - `not_null`: 非空验证
/// - `length = N`: 固定长度验证
/// - `length_range(min = N, max = M)`: 长度范围验证
/// - `exist_length = N`: 存在时的固定长度验证
/// - `exist_length_range(min = N, max = M)`: 存在时的长度范围验证
/// - `date_format = Format`: 日期格式验证
/// - `min = N`: 最小值验证
/// - `max = N`: 最大值验证
/// - `number_min = N`: 数值最小值验证
/// - `number_max = N`: 数值最大值验证
/// - `desc = "描述"`: 字段描述
///
/// # 示例
///
/// ```rust
/// use common_validation::{Validatable, DateTimeFormatEnum};
///
/// #[derive(ValidatableImpl)]
/// struct User {
///     #[validate(not_null, length_range(min = 3, max = 20), desc = "用户名")]
///     username: String,
///     
///     #[validate(not_null, date_format = Year, desc = "生日")]
///     birthday: String,
/// }
/// ```
#[proc_macro_derive(ValidatableImpl, attributes(validate))]
pub fn derive_validatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => {
                return syn::Error::new_spanned(&input, "不支持未命名字段的元组结构体")
                    .to_compile_error()
                    .into();
            },
            Fields::Unit => {
                return syn::Error::new_spanned(&input, "不支持无字段的单元结构体")
                    .to_compile_error()
                    .into();
            },
        },
        Data::Enum(_) => {
            return syn::Error::new_spanned(&input, "不支持枚举类型")
                .to_compile_error()
                .into();
        },
        Data::Union(_) => {
            return syn::Error::new_spanned(&input, "不支持联合体")
                .to_compile_error()
                .into();
        },
    };

    // 为每个字段生成验证代码
    let field_validations = fields.iter().filter_map(|f| {
        // 获取字段名
        let field_name = f.ident.as_ref()?;
        // 获取字段类型
        let field_ty = &f.ty;
        let field_ident_str = field_name.to_string();

        // 查找 validate 属性
        let validate_attr = f.attrs.iter().find(|attr| attr.path().is_ident("validate"));

        // 先收集各类规则
        let mut desc = field_ident_str.clone();
        // 先收集各类规则
        let mut not_null_rule = None;
        let mut date_format_rule = None;
        let mut length_rules = Vec::new();
        let mut number_rules = Vec::new();
        let mut structure_rule = None;
        let mut length_range = Option::<(usize, usize)>::None;
        let mut pending_length_range: Option<(usize, usize)> = None;
        let mut number_min = Option::<i64>::None;
        let mut number_max = Option::<i64>::None;

        // 类型辅助判断
        let is_string = is_string_type(field_ty);
        let is_number = is_number_type(field_ty);
        let is_vec = is_type_of(field_ty, "Vec");

        // 解析 validate 属性
        if let Some(validate_attr) = validate_attr {
            let _ = validate_attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("not_null") {
                    not_null_rule = Some(quote! { ValidationRulesEnum::NotNull });
                } else if meta.path.is_ident("length") && (is_string || is_vec) {
                    length_rules.push(quote! { ValidationRulesEnum::Length });
                } else if meta.path.is_ident("exist_length") && (is_string || is_vec) {
                    length_rules.push(quote! { ValidationRulesEnum::ExistLength });
                } else if meta.path.is_ident("length_range") && (is_string || is_vec) {
                    // 解析 length_range(min = x, max = y)
                    let mut min = None;
                    let mut max = None;
                    meta.parse_nested_meta(|meta2| {
                        if meta2.path.is_ident("min") {
                            let value = meta2.value()?;
                            let lit: LitInt = value.parse()?;
                            min = Some(lit.base10_parse::<usize>().unwrap());
                        } else if meta2.path.is_ident("max") {
                            let value = meta2.value()?;
                            let lit: LitInt = value.parse()?;
                            max = Some(lit.base10_parse::<usize>().unwrap());
                        }
                        Ok(())
                    })?;
                    if let (Some(min), Some(max)) = (min, max) {
                        pending_length_range = Some((min, max));
                    }
                } else if meta.path.is_ident("desc") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    desc = s.value();
                } else if meta.path.is_ident("min") {
                    let value = meta.value()?;
                    if let Ok(val) = value.parse::<LitInt>() {
                        if is_number {
                            number_min = val.base10_parse().ok();
                        } else if is_string || is_vec {
                            if let Ok(num) = val.base10_parse::<usize>() {
                                let current_max = match length_range {
                                    Some((_, max)) => max,
                                    None => usize::MAX,
                                };
                                length_range = Some((num, current_max));
                            }
                        }
                    }
                } else if meta.path.is_ident("max") {
                    let value = meta.value()?;
                    if let Ok(val) = value.parse::<LitInt>() {
                        if is_number {
                            number_max = val.base10_parse().ok();
                        } else if is_string || is_vec {
                            if let Ok(num) = val.base10_parse::<usize>() {
                                let current_min = match length_range {
                                    Some((min, _)) => min,
                                    None => 0,
                                };
                                length_range = Some((current_min, num));
                            }
                        }
                    }
                } else if meta.path.is_ident("date_format") {
                    let ident = meta
                        .value()?
                        .parse::<syn::Ident>()
                        .map_err(|_| meta.error("日期格式必须为枚举标识符（如 Year、DateTime、Time 等）"))?;
                    let ident_str = ident.to_string();
                    let format = match ident_str.as_str() {
                        "Time" => quote! { DateTimeFormatEnum::Time },
                        "DateTime" => quote! { DateTimeFormatEnum::DateTime },
                        "DateTimeStr" => quote! { DateTimeFormatEnum::DateTimeStr },
                        "Year" => quote! { DateTimeFormatEnum::Year },
                        "YearNoSplit" => quote! { DateTimeFormatEnum::YearNoSplit },
                        "DateTimePattern" => quote! { DateTimeFormatEnum::DateTimePattern },
                        _ => return Err(meta.error("无效的日期格式")),
                    };
                    date_format_rule = Some(quote! { ValidationRulesEnum::DateFormat(#format) });
                } else if meta.path.is_ident("number_min") && is_number {
                    let value = meta.value()?;
                    if let Ok(val) = value.parse::<LitInt>() {
                        number_min = val.base10_parse().ok();
                    }
                } else if meta.path.is_ident("number_max") && is_number {
                    let value = meta.value()?;
                    if let Ok(val) = value.parse::<LitInt>() {
                        number_max = val.base10_parse().ok();
                    }
                } else if meta.path.is_ident("structure") {
                    structure_rule = Some(quote! { ValidationRulesEnum::Structure });
                }
                Ok(())
            });
        }

        // 处理递归验证逻辑 - 如果有Structure规则或字段类型是可验证类型
        if structure_rule.is_some() {
            match (field_ty, is_type_of(field_ty, "Option"), is_type_of(field_ty, "Vec")) {
                // Option<T> 类型
                (Type::Path(ty_path), true, false) => {
                    if let Some(GenericArgument::Type(inner_ty)) = ty_path
                        .path
                        .segments
                        .last()
                        .and_then(|s| if let PathArguments::AngleBracketed(args) = &s.arguments { args.args.first().cloned() } else { None })
                    {
                        // 处理 Option<T> 和 Option<Vec<T>>
                        match (&inner_ty, is_type_of(&inner_ty, "Vec")) {
                            // Option<Vec<T>> 类型
                            (Type::Path(vec_ty), true) => {
                                if let Some(GenericArgument::Type(_vec_inner_ty)) = vec_ty
                                    .path
                                    .segments
                                    .last()
                                    .and_then(|s| if let PathArguments::AngleBracketed(args) = &s.arguments { args.args.first().cloned() } else { None })
                                {
                                    if structure_rule.is_some() {
                                        return Some(quote! {
                                            if let Some(vec_val) = &self.#field_name {
                                                for item in vec_val {
                                                    item.validate()?;
                                                }
                                            }
                                        });
                                    }
                                }
                            },
                            // Option<T> 类型
                            (_, false) => {
                                if structure_rule.is_some() {
                                    return Some(quote! {
                                        if let Some(inner) = &self.#field_name {
                                            inner.validate()?;
                                        }
                                    });
                                }
                            },
                            _ => {},
                        }
                    }
                },
                // Vec<T> 类型
                (_, false, true) => {
                    if structure_rule.is_some() {
                        return Some(quote! {
                            for item in &self.#field_name {
                                item.validate()?;
                            }
                        });
                    }
                },
                // T 类型（非Option非Vec）
                (_, false, false) => {
                    if structure_rule.is_some() {
                        return Some(quote! {
                            self.#field_name.validate()?;
                        });
                    }
                },
                // 不可能的情况（Option<Vec<T>>已经在上面处理了）
                (..) => unreachable!(),
            }
        }

        // 优先处理 length_range
        let length_range_builder = if let Some((min, max)) = pending_length_range.or(length_range) {
            if is_string || is_vec {
                length_rules.push(quote! { ValidationRulesEnum::LengthRange(#min, #max) });
            }
            quote! {}
        } else {
            quote! {}
        };

        let number_range_builder = if is_number {
            if let Some(min) = number_min {
                number_rules.push(quote! { ValidationRulesEnum::NumberMin(#min) });
            }
            if let Some(max) = number_max {
                number_rules.push(quote! { ValidationRulesEnum::NumberMax(#max) });
            }
            quote! {}
        } else {
            quote! {}
        };

        // 统一按顺序 push 规则
        let rules_builder = {
            let mut all_rules = Vec::new();
            if let Some(r) = not_null_rule {
                all_rules.push(r);
            }
            if let Some(r) = date_format_rule {
                all_rules.push(r);
            }
            if let Some(ref r) = structure_rule {
                all_rules.push(r.clone());
            }
            all_rules.extend(length_rules);
            all_rules.extend(number_rules);
            quote! {
                #(rule = rule.with_rule(#all_rules);)*
            }
        };

        // 处理 Option 类型的值访问
        let value_access = if is_type_of(field_ty, "Option") {
            // Option<String> 或 Option<Vec<T>>
            let inner_ty = extract_inner_type(field_ty);
            if let Some(inner_ty) = inner_ty {
                if is_type_of(&inner_ty, "Vec") {
                    // Option<Vec<T>>
                    quote! {
                        self.#field_name.as_ref().map(|v| v.len().to_string()).unwrap_or_default()
                    }
                } else if is_type_of(&inner_ty, "String") || is_number_type(&inner_ty) {
                    // Option<String> 或 Option<数字类型>
                    quote! {
                        self.#field_name.as_ref().map(|v| v.to_string()).unwrap_or_default()
                    }
                } else {
                    // Option<结构体类型>
                    quote! { String::new() }
                }
            } else {
                quote! {
                    self.#field_name.as_ref().map(|v| v.to_string()).unwrap_or_default()
                }
            }
        } else if is_type_of(field_ty, "Vec") {
            // Vec<T>
            quote! {
                self.#field_name.len().to_string()
            }
        } else if structure_rule.is_some() {
            // 对于结构体类型，不需要获取值字符串进行验证
            quote! { String::new() }
        } else if is_string || is_number {
            // String、数字等
            quote! { self.#field_name.to_string() }
        } else {
            // 对于其他类型（包括自定义结构体），使用空字符串
            quote! { String::new() }
        };

        // 构建验证规则
        let rule_builder = quote! {
            let mut rule = ValidationRule::new(#desc);
        };

        // 生成最终验证代码，保证规则顺序：not_null -> date_format -> structure -> length/length_range -> number_range
        Some(quote! {
            {
                #rule_builder
                #rules_builder
                #length_range_builder
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

    expanded.into()
}
