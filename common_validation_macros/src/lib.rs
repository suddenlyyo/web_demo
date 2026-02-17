//! # Common Validation Macros 验证宏库
//!
//! 提供用于自动生成验证代码的派生宏。
//!
//! ## 功能特性
//!
//! - 为结构体自动生成 [`Validatable`](common_validation::Validatable) 实现
//! - 支持多种验证属性（not_null、length、date_format等）
//! - 自动处理嵌套结构体的验证
//! - 支持Option和Vec类型的验证
//! - 支持集中管理分组验证
//!
//! ## 使用示例
//!
//! ```rust
//! use common_validation::{Validatable, DateTimeFormatEnum, ValidationErrorEnum, ValidationRule, ValidationRulesEnum, ParameterValidator, Create, Update};
//! use common_validation_macros::ValidatableImpl;
//!
//! #[derive(ValidatableImpl)]
//! #[group_fields(
//!     create = ["username", "email"],
//!     update = ["id", "username"]
//! )]
//! struct User {
//!     #[validate(not_null, length_range(min = 3, max = 20), desc = "用户名")]
//!     username: String,
//!     
//!     #[validate(not_null, desc = "用户ID")]
//!     id: String,
//!     
//!     #[validate(not_null, desc = "邮箱")]
//!     email: String,
//!     
//!     #[validate(min = 0, max = 150, desc = "年龄")]
//!     age: u8,
//! }
//!
//! let user = User {
//!     username: "test".to_string(),
//!     id: "1".to_string(),
//!     email: "test@example.com".to_string(),
//!     age: 30,
//! };
//!
//! assert!(user.validate().is_ok());
//!
//! // 分组验证
//! assert!(user.validate_with_group::<Create>().is_ok());
//! assert!(user.validate_with_group::<Update>().is_ok());
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, LitInt, LitStr, PathArguments, Type, parse_macro_input};

/// 判断给定类型是否匹配目标类型名称
///
/// 本函数检查类型是否为路径类型（如 `String` 或 `std::vec::Vec`），
/// 且路径的最后一段标识符是否与 `type_name` 相同。
/// 注意：不处理泛型参数（如 `Vec<i32>` 只会检查 `Vec` 部分）
///
/// # 参数
/// - `ty`: 要检查的类型（`syn::Type` 类型）
/// - `type_name`: 目标类型名称（如 `"String"`）
///
/// # 返回值
/// 当类型为路径类型且最后段匹配时返回 `true`，否则返回 `false`
///
/// # 示例
/// ```
/// use syn::{parse_quote, Type};
///
/// let ty: Type = parse_quote!(String);
/// assert!(is_type_of(&ty, "String"));
/// ```
fn is_type_of(ty: &Type, type_name: &str) -> bool {
    // 若非路径类型则直接返回
    let Type::Path(type_path) = ty else {
        return false;
    };

    // 获取路径最后一段，若无则返回 false
    type_path
        .path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == type_name)
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
    let Type::Path(type_path) = ty else {
        return false;
    };
    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    matches!(segment.ident.to_string().as_str(), "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize" | "f32" | "f64")
}

/// 从泛型类型中提取内部类型参数
///
/// 本函数用于解析类似 `Container<T>` 这样的泛型类型，并返回其第一个类型参数 `T`。
/// 适用于处理 `Vec<T>`、`Option<T>`、`Result<T, E>` 等常见泛型类型。
///
/// # 参数
/// - `ty`: 要解析的类型，通常是 `syn::Type` 的实例
///
/// # 返回值
/// - `Some(Type)`: 成功提取到内部类型时返回（如 `Vec<i32>` 返回 `i32`）
/// - `None`: 当输入类型不是泛型、没有类型参数或解析失败时返回
///
/// # 示例
/// ```
/// use syn::{parse_quote, Type};
///
/// // 提取 Vec<String> 中的 String
/// let ty: Type = parse_quote!(Vec<String>);
/// assert_eq!(
///     extract_inner_type(&ty).map(|t| t.to_token_stream().to_string()),
///     Some("String".to_string())
/// );
///
/// // 非泛型类型返回 None
/// let ty: Type = parse_quote!(i32);
/// assert!(extract_inner_type(&ty).is_none());
/// ```
fn extract_inner_type(ty: &Type) -> Option<Type> {
    // 第一步：匹配类型是否为路径类型（如 `std::vec::Vec`）
    // 如果不是路径类型（如原始类型 i32），直接返回 None
    let Type::Path(type_path) = ty else {
        return None;
    };

    // 第二步：获取路径的最后一段（如 `std::vec::Vec` 中的 `Vec`）
    // 如果路径为空（理论上不应该发生），返回 None
    let segment = type_path.path.segments.last()?;

    // 第三步：检查路径段是否包含尖括号泛型参数（即 `<...>` 部分）
    // 如果不是泛型实例（如单纯的 `Vec` 类型），返回 None
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    // 第四步：从泛型参数中提取第一个参数
    // 注意：这里只处理类型参数（GenericArgument::Type），忽略生命周期或常量参数
    let Some(GenericArgument::Type(inner_ty)) = args.args.first() else {
        return None;
    };

    // 返回内部类型的克隆（需要克隆因为要转移所有权）
    Some(inner_ty.clone())
}

/// 分组字段配置结构
#[derive(Default)]
struct GroupFieldsConfig {
    create_fields: Vec<String>,
    update_fields: Vec<String>,
    query_fields: Vec<String>,
    status_update_fields: Vec<String>,
    page_query_fields: Vec<String>,
}

/// 解析分组字段配置
///
/// 从结构体的 #[group_fields] 属性中解析分组字段配置
fn parse_group_fields(input: &DeriveInput) -> GroupFieldsConfig {
    let mut config = GroupFieldsConfig::default();

    for attr in &input.attrs {
        if attr.path().is_ident("group_fields") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("create") {
                    let value = meta.value()?;
                    // 直接解析为字符串数组
                    if let Ok(lit) = value.parse::<syn::LitStr>() {
                        let value = lit.value();
                        // 简单解析字符串数组格式：["field1", "field2"]
                        if value.starts_with('[') && value.ends_with(']') {
                            let fields_str = &value[1..value.len() - 1];
                            for field in fields_str.split(',') {
                                let field = field.trim();
                                if field.starts_with('"') && field.ends_with('"') {
                                    let field = &field[1..field.len() - 1];
                                    config.create_fields.push(field.to_string());
                                }
                            }
                        }
                    }
                } else if meta.path.is_ident("update") {
                    let value = meta.value()?;
                    if let Ok(lit) = value.parse::<syn::LitStr>() {
                        let value = lit.value();
                        if value.starts_with('[') && value.ends_with(']') {
                            let fields_str = &value[1..value.len() - 1];
                            for field in fields_str.split(',') {
                                let field = field.trim();
                                if field.starts_with('"') && field.ends_with('"') {
                                    let field = &field[1..field.len() - 1];
                                    config.update_fields.push(field.to_string());
                                }
                            }
                        }
                    }
                } else if meta.path.is_ident("query") {
                    let value = meta.value()?;
                    if let Ok(lit) = value.parse::<syn::LitStr>() {
                        let value = lit.value();
                        if value.starts_with('[') && value.ends_with(']') {
                            let fields_str = &value[1..value.len() - 1];
                            for field in fields_str.split(',') {
                                let field = field.trim();
                                if field.starts_with('"') && field.ends_with('"') {
                                    let field = &field[1..field.len() - 1];
                                    config.query_fields.push(field.to_string());
                                }
                            }
                        }
                    }
                } else if meta.path.is_ident("status_update") {
                    let value = meta.value()?;
                    if let Ok(lit) = value.parse::<syn::LitStr>() {
                        let value = lit.value();
                        if value.starts_with('[') && value.ends_with(']') {
                            let fields_str = &value[1..value.len() - 1];
                            for field in fields_str.split(',') {
                                let field = field.trim();
                                if field.starts_with('"') && field.ends_with('"') {
                                    let field = &field[1..field.len() - 1];
                                    config.status_update_fields.push(field.to_string());
                                }
                            }
                        }
                    }
                } else if meta.path.is_ident("page_query") {
                    let value = meta.value()?;
                    if let Ok(lit) = value.parse::<syn::LitStr>() {
                        let value = lit.value();
                        if value.starts_with('[') && value.ends_with(']') {
                            let fields_str = &value[1..value.len() - 1];
                            for field in fields_str.split(',') {
                                let field = field.trim();
                                if field.starts_with('"') && field.ends_with('"') {
                                    let field = &field[1..field.len() - 1];
                                    config.page_query_fields.push(field.to_string());
                                }
                            }
                        }
                    }
                }
                Ok(())
            });
        }
    }

    config
}

/// 实现 Validatable trait 的派生宏
///
/// 为结构体自动生成 [`Validatable`](common_validation::Validatable) trait 的实现，支持各种验证属性。
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
/// - `positive_number`: 正数验证
/// - `non_negative_number`: 非负数验证
/// - `integer`: 整数验证
/// - `decimal_scale = N`: 小数位数验证
/// - `odd_number`: 奇数验证
/// - `even_number`: 偶数验证
/// - `multiple_of = N`: 倍数验证
/// - `desc = "描述"`: 字段描述
/// - `nested`: 嵌套结构体验证（用于标记需要递归验证的结构体字段）
///
/// # 分组配置
///
/// 可以使用 #[group_fields] 属性配置分组验证字段：
///
/// ```rust
/// #[group_fields(
///     create = ["username", "email"],
///     update = ["id", "username"]
/// )]
/// ```
///
/// # 示例
///
/// ```rust
/// use common_validation::{Validatable, DateTimeFormatEnum, ValidationErrorEnum, ValidationRule, ValidationRulesEnum, ParameterValidator, Create, Update};
/// use common_validation_macros::ValidatableImpl;
///
/// #[derive(ValidatableImpl)]
/// #[group_fields(
///     create = ["username", "email"],
///     update = ["id", "username"]
/// )]
/// struct User {
///     #[validate(not_null, length_range(min = 3, max = 20), desc = "用户名")]
///     username: String,
///     
///     #[validate(not_null, desc = "用户ID")]
///     id: String,
///     
///     #[validate(not_null, desc = "邮箱")]
///     email: String,
/// }
/// ```
#[proc_macro_derive(ValidatableImpl, attributes(validate, group_fields))]
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

    // 解析分组字段配置
    let config = parse_group_fields(&input);
    let create_fields = config.create_fields;
    let update_fields = config.update_fields;
    let query_fields = config.query_fields;
    let status_update_fields = config.status_update_fields;
    let page_query_fields = config.page_query_fields;

    // 为每个字段生成验证代码
    let field_validations = fields
        .iter()
        .filter_map(|f| {
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
            let mut nested_rule = false;
            let mut length_range = Option::<(usize, usize)>::None;
            let mut number_min = Option::<i64>::None;
            let mut number_max = Option::<i64>::None;

            // 类型辅助判断
            let is_string = is_type_of(field_ty, "String") || is_type_of(field_ty, "str");
            let is_number = is_number_type(field_ty);
            let is_vec = is_type_of(field_ty, "Vec");

            // 解析 validate 属性
            if let Some(validate_attr) = validate_attr {
                let _ = validate_attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("not_null") {
                        not_null_rule = Some(quote! { common_validation::ValidationRulesEnum::NotNull });
                    } else if meta.path.is_ident("length") && (is_string || is_vec) {
                        length_rules.push(quote! { common_validation::ValidationRulesEnum::Length });
                    } else if meta.path.is_ident("exist_length") && (is_string || is_vec) {
                        length_rules.push(quote! { common_validation::ValidationRulesEnum::ExistLength });
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
                            length_range = Some((min, max));
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
                            } else if (is_string || is_vec)
                                && let Ok(num) = val.base10_parse::<usize>()
                            {
                                let current_max = match length_range {
                                    Some((_, max)) => max,
                                    None => usize::MAX,
                                };
                                length_range = Some((num, current_max));
                            }
                        }
                    } else if meta.path.is_ident("max") {
                        let value = meta.value()?;
                        if let Ok(val) = value.parse::<LitInt>() {
                            if is_number {
                                number_max = val.base10_parse().ok();
                            } else if (is_string || is_vec)
                                && let Ok(num) = val.base10_parse::<usize>()
                            {
                                let current_min = match length_range {
                                    Some((min, _)) => min,
                                    None => 0,
                                };
                                length_range = Some((current_min, num));
                            }
                        }
                    } else if meta.path.is_ident("date_format") {
                        let ident = meta
                            .value()?
                            .parse::<syn::Ident>()
                            .map_err(|_| meta.error("日期格式必须为枚举标识符（如 Year、DateTime、Time 等）"))?;
                        let ident_str = ident.to_string();
                        let format = match ident_str.as_str() {
                            "Time" => quote! { common_validation::DateTimeFormatEnum::Time },
                            "DateTime" => quote! { common_validation::DateTimeFormatEnum::DateTime },
                            "DateTimeStr" => quote! { common_validation::DateTimeFormatEnum::DateTimeStr },
                            "Year" => quote! { common_validation::DateTimeFormatEnum::Year },
                            "YearNoSplit" => quote! { common_validation::DateTimeFormatEnum::YearNoSplit },
                            "DateTimePattern" => quote! { common_validation::DateTimeFormatEnum::DateTimePattern },
                            _ => return Err(meta.error("无效的日期格式")),
                        };
                        date_format_rule = Some(quote! { common_validation::ValidationRulesEnum::DateFormat(#format) });
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
                    } else if meta.path.is_ident("positive_number") && is_number {
                        number_rules.push(quote! { common_validation::ValidationRulesEnum::PositiveNumber });
                    } else if meta.path.is_ident("non_negative_number") && is_number {
                        number_rules.push(quote! { common_validation::ValidationRulesEnum::NonNegativeNumber });
                    } else if meta.path.is_ident("integer") && is_number {
                        number_rules.push(quote! { common_validation::ValidationRulesEnum::Integer });
                    } else if meta.path.is_ident("decimal_scale") && is_number {
                        let value = meta.value()?;
                        if let Ok(val) = value.parse::<LitInt>() {
                            let scale = val.base10_parse::<u32>().unwrap();
                            number_rules.push(quote! { common_validation::ValidationRulesEnum::DecimalScale(#scale) });
                        }
                    } else if meta.path.is_ident("odd_number") && is_number {
                        number_rules.push(quote! { common_validation::ValidationRulesEnum::OddNumber });
                    } else if meta.path.is_ident("even_number") && is_number {
                        number_rules.push(quote! { common_validation::ValidationRulesEnum::EvenNumber });
                    } else if meta.path.is_ident("multiple_of") && is_number {
                        let value = meta.value()?;
                        if let Ok(val) = value.parse::<LitInt>() {
                            let multiple = val.base10_parse::<i64>().unwrap();
                            number_rules.push(quote! { common_validation::ValidationRulesEnum::MultipleOf(#multiple) });
                        }
                    } else if meta.path.is_ident("nested") {
                        nested_rule = true;
                    }
                    Ok(())
                });
            }

            // 处理递归验证逻辑 - 如果有Nested规则
            if nested_rule {
                // 对于Option<T>类型，需要特殊处理
                if is_type_of(field_ty, "Option") {
                    return Some((
                        field_ident_str,
                        quote! {
                            if let Some(ref val) = self.#field_name {
                                val.validate()?;
                            }
                        },
                    ));
                }
                // 对于Vec<T>类型，需要遍历每个元素进行验证
                else if is_type_of(field_ty, "Vec") {
                    return Some((
                        field_ident_str,
                        quote! {
                            for item in &self.#field_name {
                                item.validate()?;
                            }
                        },
                    ));
                }
                // 对于普通类型
                else {
                    return Some((
                        field_ident_str,
                        quote! {
                            self.#field_name.validate()?;
                        },
                    ));
                }
            }

            // 处理 length_range
            if let Some((min, max)) = length_range
                && (is_string || is_vec)
            {
                length_rules.push(quote! { common_validation::ValidationRulesEnum::LengthRange(#min, #max) });
            }

            if is_number {
                if let Some(min) = number_min {
                    number_rules.push(quote! { common_validation::ValidationRulesEnum::NumberMin(#min) });
                }
                if let Some(max) = number_max {
                    number_rules.push(quote! { common_validation::ValidationRulesEnum::NumberMax(#max) });
                }
            }

            // 统一按顺序 push 规则
            let rules_builder = {
                let mut all_rules = Vec::new();
                if let Some(r) = not_null_rule {
                    all_rules.push(r);
                }
                if let Some(r) = date_format_rule {
                    all_rules.push(r);
                }
                // 添加Nested规则
                if nested_rule {
                    all_rules.push(quote! { common_validation::ValidationRulesEnum::Nested });
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
                    } else if is_type_of(&inner_ty, "String") || is_type_of(&inner_ty, "str") || is_number_type(&inner_ty) {
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
            } else if nested_rule {
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
                let mut rule = common_validation::ValidationRule::new(#desc);
            };

            // 生成最终验证代码，保证规则顺序：not_null -> date_format -> nested -> length/length_range -> number_range
            Some((
                field_ident_str,
                quote! {
                    {
                        #rule_builder
                        #rules_builder
                        let value = #value_access;
                        common_validation::ParameterValidator::validate_value(&value, &rule)?;
                    }
                },
            ))
        })
        .collect::<Vec<_>>();

    // 提取验证代码
    let field_validation_codes = field_validations.iter().map(|(_, validation)| validation);

    // 生成字段验证映射
    let field_validation_mappings = field_validations.iter().map(|(field_name, validation)| {
        let field_name_lit = field_name.clone();
        quote! {
            #field_name_lit => { #validation },
        }
    });

    // 生成分组字段数组
    let create_fields_array = create_fields.iter().map(|field| quote! { #field });
    let update_fields_array = update_fields.iter().map(|field| quote! { #field });
    let query_fields_array = query_fields.iter().map(|field| quote! { #field });
    let status_update_fields_array = status_update_fields.iter().map(|field| quote! { #field });
    let page_query_fields_array = page_query_fields.iter().map(|field| quote! { #field });

    // 生成完整的 impl 块
    let expanded = quote! {
        impl #generics common_validation::Validatable for #struct_name #generics {
            fn validate(&self) -> Result<(), common_validation::ValidationErrorEnum> {
                #(#field_validation_codes)*
                Ok(())
            }
        }

        impl #generics common_validation::GroupFieldsProvider for #struct_name #generics {
            fn get_create_group_fields() -> &'static [&'static str] {
                &[#(#create_fields_array),*]
            }

            fn get_update_group_fields() -> &'static [&'static str] {
                &[#(#update_fields_array),*]
            }

            fn get_query_group_fields() -> &'static [&'static str] {
                &[#(#query_fields_array),*]
            }

            fn get_status_update_group_fields() -> &'static [&'static str] {
                &[#(#status_update_fields_array),*]
            }

            fn get_page_query_group_fields() -> &'static [&'static str] {
                &[#(#page_query_fields_array),*]
            }

            fn get_group_fields(group_name: &str) -> &'static [&'static str] {
                match group_name {
                    "create" => Self::get_create_group_fields(),
                    "update" => Self::get_update_group_fields(),
                    "query" => Self::get_query_group_fields(),
                    "status_update" => Self::get_status_update_group_fields(),
                    "page_query" => Self::get_page_query_group_fields(),
                    _ => &[],
                }
            }
        }

        impl #generics common_validation::GroupValidatable for #struct_name #generics {
            fn validate_with_group<G: common_validation::ValidationGroup>(&self) -> Result<(), common_validation::ValidationErrorEnum> {
                let group_name = G::group_name();
                let fields = <Self as common_validation::GroupFieldsProvider>::get_group_fields(group_name);

                for field in fields {
                    match *field {
                        #(#field_validation_mappings)*
                        _ => {},
                    }
                }

                Ok(())
            }
        }
    };

    expanded.into()
}
