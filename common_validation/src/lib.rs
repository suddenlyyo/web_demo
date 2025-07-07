use crate::enums::DateTimeFormatEnum;
use crate::enums::ValidateRulesEnum;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Fields, Ident, ItemFn, Lit, parse_macro_input, parse_quote,
};

mod enums;

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
        let field_name_str = field_name.to_string();
        let config = parse_field_attributes(&field.attrs, &field_name_str);
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
                            _ => panic!("未知的验证规则!"),
                        };
                        config.rules.push(rule);
                        Ok(())
                    })?;
                }

                if meta.path.is_ident("length") {
                    // 修复：延长 length_str 的生命周期
                    if let Ok(Lit::Str(length)) = meta.value().and_then(|v| v.parse()) {
                        let length_str = length.value(); // 存储为局部变量
                        let parts: Vec<&str> = length_str.split('~').collect();
                        if parts.len() == 2 {
                            config.length = Some((
                                parts[0].parse().unwrap(),
                                parts[1].parse().unwrap()
                            ));
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
                        config.number_min = min.base10_parse().unwrap();
                    }
                }

                if meta.path.is_ident("number_max") {
                    if let Ok(Lit::Int(max)) = meta.value().and_then(|v| v.parse()) {
                        config.number_max = max.base10_parse().unwrap();
                    }
                }

                Ok(())
            }).unwrap();
        }
    }

    config
}

// 修改返回类型为 TokenStream2
fn generate_field_validation(
    field_name: &Ident,
    config: &FieldValidation,
) -> TokenStream2 {
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
            // 添加对 Structure 规则的处理
            ValidateRulesEnum::Structure => {
                quote! {
                    // 嵌套结构体验证
                    #field_value.validate()?;
                }
            }
        };
        checks.push(check);
    }

    quote! {
        #(#checks)*
    }
}

#[proc_macro_attribute]
pub fn validate_parameters(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let body = &input.block;

    let expanded = quote! {
        self.validate()?;
    };

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
    use crate::enums::{DateTimeFormatEnum, ValidateRulesEnum};
    use chrono::NaiveDate;
     
    // 测试结构体
    #[derive(Validate)]
    struct User {
        #[validation(desc = "用户名", rules(NotNone, LENGTH), length = "3~20")]
        username: String,
        
        #[validation(desc = "年龄", rules(NUMBER_MIN, NUMBER_MAX), number_min = 1, number_max = 150)]
        age: String,
        
        #[validation(desc = "出生日期", rules(DATE), date_format = "DATE")]
        birthday: String,
        
        #[validation(desc = "个人简介", rules(ExistLength), length = "10~200")]
        bio: String,
        
        #[validation(desc = "地址", rules(Structure))]
        address: Address,
    }
    
    // 嵌套结构体
    #[derive(Validate)]
    struct Address {
        #[validation(desc = "省份", rules(NotNone))]
        province: String,
        
        #[validation(desc = "城市", rules(NotNone))]
        city: String,
    }
    
    #[test]
    fn test_valid_user() {
        let user = User {
            username: "john_doe".to_string(),
            age: "25".to_string(),
            birthday: "1998-05-15".to_string(),
            bio: "这是一个足够长的个人简介，超过10个字符".to_string(),
            address: Address {
                province: "广东省".to_string(),
                city: "深圳市".to_string(),
            },
        };
        
        assert_eq!(user.validate(), Ok(()));
    }
    
    #[test]
    fn test_username_validation() {
        // 测试用户名不能为空
        let user = User {
            username: "".to_string(),
            age: "25".to_string(),
            birthday: "1998-05-15".to_string(),
            bio: "足够长的简介".to_string(),
            address: Address {
                province: "广东".to_string(),
                city: "深圳".to_string(),
            },
        };
        assert_eq!(
            user.validate(),
            Err(ValidationError::NotNone("用户名".to_string()))
        );
        
        // 测试用户名长度过短
        let user = User {
            username: "ab".to_string(), // 小于3
            ..valid_user_base()
        };
        assert!(matches!(
            user.validate(),
            Err(ValidationError::Length(_, msg)) if msg.contains("3~20")
        ));
        
        // 测试用户名长度过长
        let user = User {
            username: "abcdefghijklmnopqrstuvwxyz".to_string(), // 大于20
            ..valid_user_base()
        };
        assert!(matches!(
            user.validate(),
            Err(ValidationError::Length(_, msg)) if msg.contains("3~20")
        ));
    }
    
    #[test]
    fn test_age_validation() {
        // 测试年龄过小
        let user = User {
            age: "0".to_string(), // 小于最小值1
            ..valid_user_base()
        };
        assert_eq!(
            user.validate(),
            Err(ValidationError::NumberMin("年龄".to_string(), 1))
        );
        
        // 测试年龄过大
        let user = User {
            age: "151".to_string(), // 大于最大值150
            ..valid_user_base()
        };
        assert_eq!(
            user.validate(),
            Err(ValidationError::NumberMax("年龄".to_string(), 150))
        );
        
        // 测试无效数字
        let user = User {
            age: "twentyfive".to_string(), // 非数字
            ..valid_user_base()
        };
        // 因为我们的验证只检查数字范围，所以非数字不会触发错误
        // 但实际应用中应该添加数字格式验证
        assert_eq!(user.validate(), Ok(()));
    }
    
    #[test]
    fn test_birthday_validation() {
        // 测试无效日期格式
        let user = User {
            birthday: "1998/05/15".to_string(), // 应该使用短横线分隔
            ..valid_user_base()
        };
        assert_eq!(
            user.validate(),
            Err(ValidationError::Format("出生日期".to_string()))
        );
        
        // 测试不可能日期
        let user = User {
            birthday: "1998-13-15".to_string(), // 无效月份
            ..valid_user_base()
        };
        assert_eq!(
            user.validate(),
            Err(ValidationError::Format("出生日期".to_string()))
        );
        
        // 测试有效日期
        let user = User {
            birthday: "2000-02-29".to_string(), // 闰年有效日期
            ..valid_user_base()
        };
        assert_eq!(user.validate(), Ok(()));
    }
    
    #[test]
    fn test_bio_validation() {
        // 测试空简介（应该通过，因为ExistLength允许为空）
        let user = User {
            bio: "".to_string(),
            ..valid_user_base()
        };
        assert_eq!(user.validate(), Ok(()));
        
        // 测试过短简介（当有内容时）
        let user = User {
            bio: "太短".to_string(), // 小于10
            ..valid_user_base()
        };
        assert!(matches!(
            user.validate(),
            Err(ValidationError::Length(_, msg)) if msg.contains("10~200")
        ));
        
        // 测试过长简介
        let user = User {
            bio: "a".repeat(201), // 大于200
            ..valid_user_base()
        };
        assert!(matches!(
            user.validate(),
            Err(ValidationError::Length(_, msg)) if msg.contains("10~200")
        ));
    }
    
    #[test]
    fn test_nested_structure_validation() {
        // 测试省份为空
        let user = User {
            address: Address {
                province: "".to_string(),
                city: "深圳".to_string(),
            },
            ..valid_user_base()
        };
        assert_eq!(
            user.validate(),
            Err(ValidationError::NotNone("省份".to_string()))
        );
        
        // 测试城市为空
        let user = User {
            address: Address {
                province: "广东".to_string(),
                city: "".to_string(),
            },
            ..valid_user_base()
        };
        assert_eq!(
            user.validate(),
            Err(ValidationError::NotNone("城市".to_string()))
        );
    }
    
    #[test]
    fn test_validation_order() {
        // 测试多个错误时，返回第一个遇到的错误
        let user = User {
            username: "".to_string(), // 错误1
            age: "0".to_string(),    // 错误2
            birthday: "invalid".to_string(), // 错误3
            bio: "太短".to_string(),  // 错误4
            address: Address {
                province: "".to_string(), // 错误5
                city: "".to_string(),     // 错误6
            },
        };
        
        // 应该返回用户名不能为空的错误（第一个字段）
        assert_eq!(
            user.validate(),
            Err(ValidationError::NotNone("用户名".to_string()))
        );
    }
    
    // 创建一个有效用户的基础数据
    fn valid_user_base() -> User {
        User {
            username: "valid_user".to_string(),
            age: "30".to_string(),
            birthday: "1993-08-20".to_string(),
            bio: "这是一个足够长的个人简介，超过10个字符".to_string(),
            address: Address {
                province: "江苏省".to_string(),
                city: "南京市".to_string(),
            },
        }
    }
}