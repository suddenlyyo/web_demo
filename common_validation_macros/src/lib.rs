use common_validation::{DateTimeFormatEnum, ValidateRulesEnum, ValidationErrorEnum};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Ident, Lit, parse_macro_input, Type};

#[proc_macro_derive(Validate, attributes(validation))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let fields = if let Data::Struct(s) = &input.data {
        &s.fields
    } else {
        panic!("Validate 仅支持结构体!");
    };

    let field_validations = match fields {
        Fields::Named(fields) => &fields.named,
        _ => panic!("仅支持具名字段!"),
    };

    let validation_calls = field_validations.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let field_name_str = field_name.to_string();
        let config = parse_field_attributes(&field.attrs, &field_name_str);
        generate_field_validation(&field_name, field_type, &config)
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn validate(&self) -> Result<(), ValidationErrorEnum> {
                #(#validation_calls)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

struct FieldValidation {
    desc: String,
    rules: Vec<ValidateRulesEnum>,
    length: Option<(usize, usize)>,
    date_format: DateTimeFormatEnum,
    number_min: i64,
    number_max: i64,
}

fn parse_field_attributes(attrs: &[Attribute], field_name: &str) -> FieldValidation {
    let mut config = FieldValidation {
        desc: field_name.to_string(),
        rules: Vec::new(),
        length: None,
        date_format: DateTimeFormatEnum::None,
        number_min: i64::MIN,
        number_max: i64::MAX,
    };

    for attr in attrs {
        if attr.path().is_ident("validation") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("desc") {
                    if let Ok(Lit::Str(desc)) = meta.value().and_then(|v| v.parse()) {
                        config.desc = desc.value();
                    }
                    return Ok(());
                }

                if meta.path.is_ident("rules") {
                    meta.parse_nested_meta(|rule_meta| {
                        let rule = match rule_meta.path.get_ident().unwrap().to_string().as_str() {
                            "NotNone" => ValidateRulesEnum::NotNone,
                            "LENGTH" => ValidateRulesEnum::Length,
                            "ExistLength" => ValidateRulesEnum::ExistLength,
                            "DATE" => ValidateRulesEnum::Date,
                            "TIME" => ValidateRulesEnum::Time,
                            "DATE_TIME" => ValidateRulesEnum::DateTime,
                            "NUMBER_MIN" => ValidateRulesEnum::NumberMin,
                            "NUMBER_MAX" => ValidateRulesEnum::NumberMax,
                            "Structure" => ValidateRulesEnum::Structure,
                            _ => panic!("未知的验证规则: {:?}", rule_meta.path),
                        };
                        config.rules.push(rule);
                        Ok(())
                    })?;
                }

                if meta.path.is_ident("length") {
                    if let Ok(Lit::Str(length)) = meta.value().and_then(|v| v.parse()) {
                        let length_str = length.value();
                        let parts: Vec<&str> = length_str.split('~').collect();
                        if parts.len() == 2 {
                            if let (Ok(min), Ok(max)) = (parts[0].parse(), parts[1].parse()) {
                                config.length = Some((min, max));
                            }
                        }
                    }
                }

                if meta.path.is_ident("date_format") {
                    if let Ok(Lit::Str(format)) = meta.value().and_then(|v| v.parse()) {
                        config.date_format = match format.value().as_str() {
                            "TIME" => DateTimeFormatEnum::Time,
                            "DATE_TIME" => DateTimeFormatEnum::DateTime,
                            "DATE_TIME_STR" => DateTimeFormatEnum::DateTimeStr,
                            "YEAR" => DateTimeFormatEnum::Year,
                            "YEAR_NO_SPLIT" => DateTimeFormatEnum::YearNoSplit,
                            "DATE_TIME_PATTERN" => DateTimeFormatEnum::DateTimePattern,
                            _ => DateTimeFormatEnum::None,
                        };
                    }
                }

                if meta.path.is_ident("number_min") {
                    if let Ok(Lit::Int(min)) = meta.value().and_then(|v| v.parse()) {
                        if let Ok(value) = min.base10_parse() {
                            config.number_min = value;
                        }
                    }
                }

                if meta.path.is_ident("number_max") {
                    if let Ok(Lit::Int(max)) = meta.value().and_then(|v| v.parse()) {
                        if let Ok(value) = max.base10_parse() {
                            config.number_max = value;
                        }
                    }
                }

                Ok(())
            }).unwrap();
        }
    }

    config
}

fn generate_field_validation(field_name: &Ident, field_type: &Type, config: &FieldValidation) -> TokenStream2 {
    let desc = &config.desc;
    let field_value = quote! { &self.#field_name };
    
    // 判断字段类型是否是 Option
    let is_option = if let Type::Path(type_path) = field_type {
        type_path.path.segments.last().map(|s| s.ident == "Option").unwrap_or(false)
    } else {
        false
    };

    let mut checks = Vec::new();

    for rule in &config.rules {
        let check = match rule {
            ValidateRulesEnum::NotNone => {
                if is_option {
                    // 处理 Option 类型
                    quote! {
                        if #field_value.is_none() {
                            return Err(ValidationErrorEnum::NotNone(#desc.to_string()));
                        }
                    }
                } else {
                    // 处理非 Option 类型（如 String）
                    quote! {
                        if #field_value.is_empty() {
                            return Err(ValidationErrorEnum::NotNone(#desc.to_string()));
                        }
                    }
                }
            }
            ValidateRulesEnum::Length => {
                if let Some((min, max)) = config.length {
                    quote! {
                        let len = #field_value.len();
                        if len < #min || len > #max {
                            return Err(ValidationErrorEnum::Length(
                                #desc.to_string(),
                                format!("长度必须在 {}~{} 之间", #min, #max)
                            ));
                        }
                    }
                } else {
                    quote! {}
                }
            }
            ValidateRulesEnum::ExistLength => {
                if let Some((min, max)) = config.length {
                    if is_option {
                        quote! {
                            if let Some(val) = #field_value {
                                if !val.is_empty() {
                                    let len = val.len();
                                    if len < #min || len > #max {
                                        return Err(ValidationErrorEnum::Length(
                                            #desc.to_string(),
                                            format!("长度必须在 {}~{} 之间", #min, #max)
                                        ));
                                    }
                                }
                            }
                        }
                    } else {
                        quote! {
                            if !#field_value.is_empty() {
                                let len = #field_value.len();
                                if len < #min || len > #max {
                                    return Err(ValidationErrorEnum::Length(
                                        #desc.to_string(),
                                        format!("长度必须在 {}~{} 之间", #min, #max)
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    quote! {}
                }
            }
            ValidateRulesEnum::Date | ValidateRulesEnum::Time | ValidateRulesEnum::DateTime => {
                let format = config.date_format.pattern();
                if is_option {
                    quote! {
                        if let Some(val) = #field_value {
                            if !val.is_empty() {
                                // 将字符串转换为 &str 进行解析
                                if chrono::NaiveDate::parse_from_str(val.as_str(), #format).is_err() {
                                    return Err(ValidationErrorEnum::Format(#desc.to_string()));
                                }
                            }
                        }
                    }
                } else {
                    quote! {
                        if !#field_value.is_empty() {
                            // 将字符串转换为 &str 进行解析
                            if chrono::NaiveDate::parse_from_str(#field_value.as_str(), #format).is_err() {
                                return Err(ValidationErrorEnum::Format(#desc.to_string()));
                            }
                        }
                    }
                }
            }
            ValidateRulesEnum::NumberMin => {
                let min = config.number_min;
                if is_option {
                    quote! {
                        if let Some(val) = #field_value {
                            if *val < #min {
                                return Err(ValidationErrorEnum::NumberMin(#desc.to_string(), #min));
                            }
                        }
                    }
                } else {
                    quote! {
                        if *#field_value < #min {
                            return Err(ValidationErrorEnum::NumberMin(#desc.to_string(), #min));
                        }
                    }
                }
            }
            ValidateRulesEnum::NumberMax => {
                let max = config.number_max;
                if is_option {
                    quote! {
                        if let Some(val) = #field_value {
                            if *val > #max {
                                return Err(ValidationErrorEnum::NumberMax(#desc.to_string(), #max));
                            }
                        }
                    }
                } else {
                    quote! {
                        if *#field_value > #max {
                            return Err(ValidationErrorEnum::NumberMax(#desc.to_string(), #max));
                        }
                    }
                }
            }
            ValidateRulesEnum::Structure => {
                if is_option {
                    quote! {
                        if let Some(val) = #field_value {
                            if let Err(e) = val.validate() {
                                return Err(e);
                            }
                        }
                    }
                } else {
                    quote! {
                        if let Err(e) = #field_value.validate() {
                            return Err(e);
                        }
                    }
                }
            }
        };
        checks.push(check);
    }

    quote! {
        #(#checks)*
    }
}