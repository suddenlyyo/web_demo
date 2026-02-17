use common_validation::{CreateGroup, DateTimeFormatEnum, PageQueryGroup, ParameterValidator, QueryGroup, StatusUpdateGroup, UpdateGroup, Validatable, ValidationErrorEnum, ValidationRule, ValidationRulesEnum};
use common_validation_macros::ValidatableImpl;

// ====================== 基本结构体验证 ======================
#[derive(Debug, ValidatableImpl)]
struct BasicUser {
    #[validate(not_null, length_range(min = 3, max = 20), desc = "用户名")]
    username: String,

    #[validate(not_null, min = 1, max = 120, desc = "年龄")]
    age: i32,

    #[validate(not_null, date_format = Year, desc = "生日")]
    birthdate: String,
}

#[test]
fn test_basic_struct_validation() {
    // 有效用户
    let valid_user = BasicUser {
        username: "john_doe".to_string(),
        age: 30,
        birthdate: "1990-01-01".to_string(),
    };
    assert!(valid_user.validate().is_ok());

    // 用户名太短
    let invalid_username = BasicUser {
        username: "jd".to_string(), // 太短
        age: 30,
        birthdate: "1990-01-01".to_string(),
    };
    assert!(matches!(
        invalid_username.validate(),
        Err(ValidationErrorEnum::Length(_, msg)) if msg.contains("3~20")
    ));

    // 年龄超出范围
    let invalid_age = BasicUser {
        username: "john_doe".to_string(),
        age: 150, // 超出最大值
        birthdate: "1990-01-01".to_string(),
    };
    assert_eq!(invalid_age.validate(), Err(ValidationErrorEnum::NumberMax("年龄".to_string(), 120)));

    // 日期格式错误
    let invalid_date = BasicUser {
        username: "john_doe".to_string(),
        age: 30,
        birthdate: "1990/01/111".to_string(), // 格式错误
    };
    assert_eq!(invalid_date.validate(), Err(ValidationErrorEnum::Format("生日".to_string())));
}

// ====================== 嵌套结构体验证 ======================
#[derive(Debug, ValidatableImpl)]
struct Address {
    #[validate(not_null, length_range(min = 5, max = 50), desc = "街道")]
    street: String,

    #[validate(not_null, desc = "城市")]
    city: String,

    #[validate(length_range(min = 5, max = 10), desc = "邮编")]
    zipcode: Option<String>,
}

#[derive(Debug, ValidatableImpl)]
struct UserProfile {
    #[validate(nested, desc = "基础信息")]
    basic: BasicUser,

    #[validate(nested, desc = "地址")]
    address: Address,

    #[validate(nested, desc = "备用地址")]
    secondary_address: Option<Address>,
}

#[test]
fn test_nested_struct_validation() {
    // 有效嵌套结构体
    let valid_profile = UserProfile {
        basic: BasicUser {
            username: "jane_doe".to_string(),
            age: 28,
            birthdate: "1995-05-15".to_string(),
        },
        address: Address {
            street: "123 Main Street".to_string(),
            city: "Metropolis".to_string(),
            zipcode: Some("12345".to_string()),
        },
        secondary_address: Some(Address {
            street: "456 Oak Avenue".to_string(),
            city: "Smallville".to_string(),
            zipcode: None,
        }),
    };
    assert!(valid_profile.validate().is_ok());

    // 嵌套结构体中的错误 - 基础信息错误
    let invalid_basic = UserProfile {
        basic: BasicUser {
            username: "jd".to_string(), // 太短
            age: 28,
            birthdate: "1995-05-15".to_string(),
        },
        address: Address {
            street: "123 Main Street".to_string(),
            city: "Metropolis".to_string(),
            zipcode: Some("12345".to_string()),
        },
        secondary_address: None,
    };
    assert!(matches!(
        invalid_basic.validate(),
        Err(ValidationErrorEnum::Length(_, msg)) if msg.contains("3~20")
    ));

    // 嵌套结构体中的错误 - 地址错误
    let invalid_address = UserProfile {
        basic: BasicUser {
            username: "jane_doe".to_string(),
            age: 28,
            birthdate: "1995-05-15".to_string(),
        },
        address: Address {
            street: "123".to_string(), // 太短
            city: "Metropolis".to_string(),
            zipcode: Some("12345".to_string()),
        },
        secondary_address: None,
    };
    assert!(matches!(
        invalid_address.validate(),
        Err(ValidationErrorEnum::Length(_, msg)) if msg.contains("5~50")
    ));

    // 可选嵌套结构体中的错误
    let invalid_secondary = UserProfile {
        basic: BasicUser {
            username: "jane_doe".to_string(),
            age: 28,
            birthdate: "1995-05-15".to_string(),
        },
        address: Address {
            street: "123 Main Street".to_string(),
            city: "Metropolis".to_string(),
            zipcode: Some("12345".to_string()),
        },
        secondary_address: Some(Address {
            street: "456 Oak".to_string(),
            city: "".to_string(), // 城市不能为空
            zipcode: None,
        }),
    };
    assert_eq!(invalid_secondary.validate(), Err(ValidationErrorEnum::NotNull("城市".to_string())));
}

// ====================== 边界情况测试 ======================
#[test]
fn test_edge_cases() {
    // 测试空字符串
    #[derive(Debug, ValidatableImpl)]
    struct EmptyTest {
        #[validate(not_null, desc = "非空字段")]
        non_empty: String,
    }

    let empty_test = EmptyTest { non_empty: "".to_string() };
    assert_eq!(empty_test.validate(), Err(ValidationErrorEnum::NotNull("非空字段".to_string())));

    // 测试Option类型的None值 - 需要添加not_null规则才会报错
    #[derive(Debug, ValidatableImpl)]
    struct OptionTest {
        #[validate(not_null, desc = "非空Option")]
        opt_field: Option<String>,
    }

    let none_test = OptionTest { opt_field: None };
    assert_eq!(none_test.validate(), Err(ValidationErrorEnum::NotNull("非空Option".to_string())));

    // 测试最小边界值
    #[derive(Debug, ValidatableImpl)]
    struct MinValueTest {
        #[validate(min = 10, desc = "最小值测试")]
        value: i32,
    }

    let min_test = MinValueTest { value: 9 };
    assert_eq!(min_test.validate(), Err(ValidationErrorEnum::NumberMin("最小值测试".to_string(), 10)));
}

// ====================== 自定义错误消息测试 ======================
#[test]
fn test_custom_error_messages() {
    #[derive(Debug, ValidatableImpl)]
    struct CustomMessageTest {
        #[validate(not_null, desc = "自定义描述字段")]
        field1: String,

        #[validate(length_range(min = 5, max = 10), desc = "长度测试字段")]
        field2: String,
    }

    // 测试自定义描述字段
    let null_test = CustomMessageTest { field1: "".to_string(), field2: "12345".to_string() };
    assert_eq!(null_test.validate(), Err(ValidationErrorEnum::NotNull("自定义描述字段".to_string())));

    // 测试长度错误消息
    let length_test = CustomMessageTest { field1: "valid".to_string(), field2: "123".to_string() };
    assert!(matches!(
        length_test.validate(),
        Err(ValidationErrorEnum::Length(_, msg)) if msg.contains("5~10")
    ));
}

// ====================== 复杂类型验证测试 ======================
#[test]
fn test_complex_types() {
    // 测试Vec类型验证 - 重点验证元素而非集合大小
    #[derive(Debug, ValidatableImpl)]
    struct VecTest {
        #[validate(not_null, desc = "字符串数组")]
        strings: Vec<String>,
    }

    // 测试有效的Vec（非空）
    let valid_vec = VecTest { strings: vec!["one".to_string(), "two".to_string()] };
    assert!(valid_vec.validate().is_ok());

    // 测试无效的Vec（null情况）
    #[derive(Debug, ValidatableImpl)]
    struct OptionalVecTest {
        #[validate(not_null, desc = "可选字符串数组")] // 添加not_null规则以确保None时报错
        strings: Option<Vec<String>>,
    }

    let none_vec = OptionalVecTest { strings: None };
    assert_eq!(none_vec.validate(), Err(ValidationErrorEnum::NotNull("可选字符串数组".to_string()))); // 现在应该报错

    // 测试嵌套Vec验证 - 这部分逻辑是合理的
    #[derive(Debug, ValidatableImpl)]
    struct NestedVecTest {
        #[validate(nested, desc = "嵌套验证")]
        items: Vec<BasicUser>,
    }

    let valid_nested = NestedVecTest {
        items: vec![
            BasicUser {
                username: "user1".to_string(),
                age: 20,
                birthdate: "2000-01-01".to_string(),
            },
            BasicUser {
                username: "user2".to_string(),
                age: 25,
                birthdate: "1995-01-01".to_string(),
            },
        ],
    };
    assert!(valid_nested.validate().is_ok());

    let invalid_nested = NestedVecTest {
        items: vec![
            BasicUser {
                username: "user1".to_string(),
                age: 20,
                birthdate: "2000-01-01".to_string(),
            },
            BasicUser {
                username: "u".to_string(), // 太短
                age: 25,
                birthdate: "1995-01-01".to_string(),
            },
        ],
    };
    assert!(matches!(
        invalid_nested.validate(),
        Err(ValidationErrorEnum::Length(_, msg)) if msg.contains("3~20")
    ));

    // 空Vec测试
    let empty_vec = NestedVecTest { items: vec![] };
    assert!(empty_vec.validate().is_ok()); // 空Vec本身是有效的，除非有特殊约束
}

// ====================== 新增验证规则测试 ======================
#[test]
fn test_new_validation_rules() {
    // 测试正数验证
    #[derive(Debug, ValidatableImpl)]
    struct PositiveTest {
        #[validate(positive_number, desc = "正数测试")]
        value: f64,
    }

    let positive_valid = PositiveTest { value: 10.5 };
    assert!(positive_valid.validate().is_ok());

    let positive_invalid = PositiveTest { value: -5.0 };
    assert_eq!(positive_invalid.validate(), Err(ValidationErrorEnum::PositiveNumber("正数测试".to_string())));

    // 测试非负数验证
    #[derive(Debug, ValidatableImpl)]
    struct NonNegativeTest {
        #[validate(non_negative_number, desc = "非负数测试")]
        value: f64,
    }

    let non_negative_valid1 = NonNegativeTest { value: 0.0 };
    assert!(non_negative_valid1.validate().is_ok());

    let non_negative_valid2 = NonNegativeTest { value: 100.0 };
    assert!(non_negative_valid2.validate().is_ok());

    let non_negative_invalid = NonNegativeTest { value: -1.0 };
    assert_eq!(non_negative_invalid.validate(), Err(ValidationErrorEnum::NonNegativeNumber("非负数测试".to_string())));

    // 测试整数验证
    #[derive(Debug, ValidatableImpl)]
    struct IntegerTest {
        #[validate(integer, desc = "整数测试")]
        value: f64,
    }

    let integer_valid = IntegerTest { value: 42.0 };
    assert!(integer_valid.validate().is_ok());

    let integer_invalid = IntegerTest { value: 42.5 };
    assert_eq!(integer_invalid.validate(), Err(ValidationErrorEnum::Integer("整数测试".to_string())));

    // 测试小数位数验证
    #[derive(Debug, ValidatableImpl)]
    struct DecimalScaleTest {
        #[validate(decimal_scale = 2, desc = "小数位数测试")]
        value: f64,
    }

    let decimal_valid1 = DecimalScaleTest { value: 123.45 };
    assert!(decimal_valid1.validate().is_ok());

    let decimal_valid2 = DecimalScaleTest { value: 123.0 };
    assert!(decimal_valid2.validate().is_ok());

    let decimal_invalid = DecimalScaleTest { value: 123.456 };
    assert_eq!(decimal_invalid.validate(), Err(ValidationErrorEnum::DecimalScale("小数位数测试".to_string(), 2)));

    // 测试奇数验证
    #[derive(Debug, ValidatableImpl)]
    struct OddTest {
        #[validate(odd_number, desc = "奇数测试")]
        value: i32,
    }

    let odd_valid = OddTest { value: 7 };
    assert!(odd_valid.validate().is_ok());

    let odd_invalid = OddTest { value: 8 };
    assert_eq!(odd_invalid.validate(), Err(ValidationErrorEnum::OddNumber("奇数测试".to_string())));

    // 测试偶数验证
    #[derive(Debug, ValidatableImpl)]
    struct EvenTest {
        #[validate(even_number, desc = "偶数测试")]
        value: i32,
    }

    let even_valid = EvenTest { value: 8 };
    assert!(even_valid.validate().is_ok());

    let even_invalid = EvenTest { value: 7 };
    assert_eq!(even_invalid.validate(), Err(ValidationErrorEnum::EvenNumber("偶数测试".to_string())));

    // 测试倍数验证
    #[derive(Debug, ValidatableImpl)]
    struct MultipleOfTest {
        #[validate(multiple_of = 5, desc = "倍数测试")]
        value: i32,
    }

    let multiple_valid = MultipleOfTest { value: 15 };
    assert!(multiple_valid.validate().is_ok());

    let multiple_invalid = MultipleOfTest { value: 16 };
    assert_eq!(multiple_invalid.validate(), Err(ValidationErrorEnum::MultipleOf("倍数测试".to_string(), 5)));
}
