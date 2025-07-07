use crate::enums::DateTimeFormatEnum;
use crate::enums::ValidateRulesEnum;
use crate::proc_macro::TokenStream;
use proc_macro;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Fields, Ident, ItemFn, Lit, parse_macro_input, parse_quote,
};

mod enums;

// 过程宏：为结构体生成验证方法,定义一个名为 Validate 的过程宏,并设置一个名为 `validation` 的属性宏。
#[proc_macro_derive(Validate, attributes(validation))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream
    let input = parse_macro_input!(input as DeriveInput);
    // 获取结构体名称
    let struct_name = &input.ident;
    // 获取结构体的数据类型
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
        let field_name_str = field_name.to_string();

        // 解析字段属性
        let config = parse_field_attributes(&field.attrs, &field_name_str);

        // 为每个字段生成验证逻辑
        generate_field_validation(&field_name, &config)
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn validate(&self) -> Result<(), ValidationError> {
                #(#validation_calls)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
/// 字段验证配置
struct FieldValidation {
    desc: String,
    rules: Vec<ValidateRulesEnum>,
    length: Option<(usize, usize)>,
    date_format: DateTimeFormatEnum,
    number_min: i64,
    number_max: i64,
}
// 解析字段属性
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
                    if let Some(Lit::Str(desc)) = meta.value().and_then(|v| v.parse().ok()) {
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
                            _ => panic!("未知的验证规则!"),
                        };
                        config.rules.push(rule);
                        Ok(())
                    })?;
                }

                if meta.path.is_ident("length") {
                    if let Some(Lit::Str(length)) = meta.value().and_then(|v| v.parse().ok()) {
                        let parts: Vec<&str> = length.value().split('~').collect();
                        if parts.len() == 2 {
                            config.length =
                                Some((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
                        }
                    }
                }

                if meta.path.is_ident("date_format") {
                    if let Some(Lit::Str(format)) = meta.value().and_then(|v| v.parse().ok()) {
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
                    if let Some(Lit::Int(min)) = meta.value().and_then(|v| v.parse().ok()) {
                        config.number_min = min.base10_parse().unwrap();
                    }
                }

                if meta.path.is_ident("number_max") {
                    if let Some(Lit::Int(max)) = meta.value().and_then(|v| v.parse().ok()) {
                        config.number_max = max.base10_parse().unwrap();
                    }
                }

                Ok(())
            })
            .unwrap();
        }
    }

    config
}

// 为字段生成验证代码
fn generate_field_validation(
    field_name: &Ident,
    config: &FieldValidation,
) -> proc_macro::TokenStream {
    let desc = &config.desc;
    let field_value = quote! { &self.#field_name };

    let mut checks = Vec::new();

    for rule in &config.rules {
        let check = match rule {
            ValidateRulesEnum::NotNone => {
                quote! {
                    if #field_value.is_empty() {
                        return Err(ValidationError::NotNone(#desc.to_string()));
                    }
                }
            }
            ValidateRulesEnum::Length => {
                if let Some((min, max)) = config.length {
                    quote! {
                        let len = #field_value.len();
                        if len < #min || len > #max {
                            return Err(ValidationError::Length(
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
                    quote! {
                        if !#field_value.is_empty() {
                            let len = #field_value.len();
                            if len < #min || len > #max {
                                return Err(ValidationError::Length(
                                    #desc.to_string(),
                                    format!("长度必须在 {}~{} 之间", #min, #max)
                                ));
                            }
                        }
                    }
                } else {
                    quote! {}
                }
            }
            ValidateRulesEnum::Date | ValidateRulesEnum::Time | ValidateRulesEnum::DateTime => {
                let format = config.date_format.pattern();
                quote! {
                    if !#field_value.is_empty() {
                        if chrono::NaiveDate::parse_from_str(#field_value, #format).is_err() {
                            return Err(ValidationError::Format(#desc.to_string()));
                        }
                    }
                }
            }
            ValidateRulesEnum::NumberMin => {
                let min = config.number_min;
                quote! {
                    if let Ok(num) = #field_value.parse::<i64>() {
                        if num < #min {
                            return Err(ValidationError::NumberMin(#desc.to_string(), #min));
                        }
                    }
                }
            }
            ValidateRulesEnum::NumberMax => {
                let max = config.number_max;
                quote! {
                    if let Ok(num) = #field_value.parse::<i64>() {
                        if num > #max {
                            return Err(ValidationError::NumberMax(#desc.to_string(), #max));
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
// 函数验证属性宏
#[proc_macro_attribute]
pub fn validate_parameters(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let body = &input.block;
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;

    // 生成验证代码
    let expanded = quote! {
        for param in [&self] {
            param.validate()?;
        }
    };

    // 修改函数体
    input.block = parse_quote! {
        {
            #expanded
            #body
        }
    };

    TokenStream::from(quote! {
        #input
    })
}

#[cfg(test)]
mod tests {
    use super::*;
}
