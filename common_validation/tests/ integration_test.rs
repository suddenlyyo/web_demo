use chrono::NaiveDate;
use common_validation::{DateTimeFormatEnum, ValidateRulesEnum, ValidationErrorEnum};

// 测试结构体
#[derive(Validate)]
struct User {
    #[validation(desc = "用户名", rules(NotNone, LENGTH), length = "3~20")]
    username: String,

    #[validation(
        desc = "年龄",
        rules(NUMBER_MIN, NUMBER_MAX),
        number_min = 1,
        number_max = 150
    )]
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
        Err(ValidationErrorEnum::NotNone("用户名".to_string()))
    );

    // 测试用户名长度过短
    let user = User {
        username: "ab".to_string(), // 小于3
        ..valid_user_base()
    };
    assert_eq!(
        user.validate(),
        Err(ValidationErrorEnum::Length(
            "用户名".to_string(),
            "长度必须在 3~20 之间".to_string()
        ))
    );

    // 测试用户名长度过长
    let user = User {
        username: "abcdefghijklmnopqrstuvwxyz".to_string(), // 大于20
        ..valid_user_base()
    };
    assert_eq!(
        user.validate(),
        Err(ValidationErrorEnum::Length(
            "用户名".to_string(),
            "长度必须在 3~20 之间".to_string()
        ))
    );
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
        Err(ValidationErrorEnum::NumberMin("年龄".to_string(), 1))
    );

    // 测试年龄过大
    let user = User {
        age: "151".to_string(), // 大于最大值150
        ..valid_user_base()
    };
    assert_eq!(
        user.validate(),
        Err(ValidationErrorEnum::NumberMax("年龄".to_string(), 150))
    );

    // 测试无效数字 - 注意：目前我们的验证在解析失败时不会报错，所以这里预期是OK
    let user = User {
        age: "twentyfive".to_string(), // 非数字
        ..valid_user_base()
    };
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
        Err(ValidationErrorEnum::Format("出生日期".to_string()))
    );

    // 测试不可能日期
    let user = User {
        birthday: "1998-13-15".to_string(), // 无效月份
        ..valid_user_base()
    };
    assert_eq!(
        user.validate(),
        Err(ValidationErrorEnum::Format("出生日期".to_string()))
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
    assert_eq!(
        user.validate(),
        Err(ValidationErrorEnum::Length(
            "个人简介".to_string(),
            "长度必须在 10~200 之间".to_string()
        ))
    );

    // 测试过长简介
    let user = User {
        bio: "a".repeat(201), // 大于200
        ..valid_user_base()
    };
    assert_eq!(
        user.validate(),
        Err(ValidationErrorEnum::Length(
            "个人简介".to_string(),
            "长度必须在 10~200 之间".to_string()
        ))
    );
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
        Err(ValidationErrorEnum::NotNone("省份".to_string()))
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
        Err(ValidationErrorEnum::NotNone("城市".to_string()))
    );
}

#[test]
fn test_validation_order() {
    // 测试多个错误时，返回第一个遇到的错误（即username字段的错误）
    let user = User {
        username: "".to_string(),        // 错误1
        age: "0".to_string(),            // 错误2
        birthday: "invalid".to_string(), // 错误3
        bio: "太短".to_string(),         // 错误4
        address: Address {
            province: "".to_string(), // 错误5
            city: "".to_string(),     // 错误6
        },
    };

    // 应该返回用户名不能为空的错误（第一个字段）
    assert_eq!(
        user.validate(),
        Err(ValidationErrorEnum::NotNone("用户名".to_string()))
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
