use common_validation::{DateTimeFormatEnum, ParameterValidator, Validatable, ValidationErrorEnum, ValidationRule};

#[derive(Debug)]
struct User {
    username: String,
    age: String,
    birthdate: String,
}

impl Validatable for User {
    fn validate(&self) -> Result<(), ValidationErrorEnum> {
        // 用户名验证规则
        let username_rule = ValidationRule::new("用户名").not_null().length_range(3, 20); // 长度范围3-20

        ParameterValidator::validate_value(&self.username, &username_rule)?;

        // 年龄验证规则
        let age_rule = ValidationRule::new("年龄")
            .not_null()
            .min(1) // 最小值1
            .max(120); // 最大值120   

        ParameterValidator::validate_value(&self.age, &age_rule)?;

        // 生日验证规则
        let birthdate_rule = ValidationRule::new("生日")
            .not_null()
            .date_format(DateTimeFormatEnum::Year);

        ParameterValidator::validate_value(&self.birthdate, &birthdate_rule)?;
        Ok(())
    }
}

// 打印输出测试命令: cargo test -p common_validation --test integration_test -- --show-output
#[test]
fn validate_test() {
    // 创建测试用户
    let valid_user = User {
        username: "john_doe".to_string(),    // 长度8，在3-20范围内
        age: "30".to_string(),               // 在1-120范围内
        birthdate: "1990-01-01".to_string(), // 正确格式
    };

    let invalid_user = User {
        username: "jd".to_string(),          // 长度2，小于最小值3
        age: "150".to_string(),              // 超过最大值120
        birthdate: "1990/01/01".to_string(), // 格式错误
    };

    // 验证有效用户
    assert!(valid_user.validate().is_ok(), "有效用户应该验证通过");

    // 验证无效用户
    let invalid_result = invalid_user.validate();
    assert!(invalid_result.is_err(), "无效用户应该验证失败");

    if let Err(e) = invalid_result {
        println!("无效用户验证失败(符合预期): {}", e);
    }

    // 直接值验证示例
    let email_rule = ValidationRule::new("邮箱").not_null().length_range(5, 100); // 长度范围5-100

    let email_result = ParameterValidator::validate_value("test@example.com", &email_rule);
    assert!(email_result.is_ok(), "有效邮箱应该验证通过");

    if let Ok(_) = email_result {
        println!("邮箱验证通过(符合预期)");
    }
}

#[test]
fn empty_username_should_fail() {
    let user = User {
        username: "".to_string(),
        age: "25".to_string(),
        birthdate: "2000-01-01".to_string(),
    };
    let result = user.validate();
    assert!(result.is_err(), "空用户名应该验证失败");
    if let Err(e) = result {
        println!("空用户名验证失败(符合预期): {}", e);
    }
}

#[test]
fn empty_age_should_fail() {
    let user = User {
        username: "validuser".to_string(),
        age: "".to_string(),
        birthdate: "2000-01-01".to_string(),
    };
    let result = user.validate();
    assert!(result.is_err(), "空年龄应该验证失败");
    if let Err(e) = result {
        println!("空年龄验证失败(符合预期): {}", e);
    }
}

#[test]
fn empty_birthdate_should_fail() {
    let user = User {
        username: "validuser".to_string(),
        age: "25".to_string(),
        birthdate: "".to_string(),
    };
    let result = user.validate();
    assert!(result.is_err(), "空生日应该验证失败");
    if let Err(e) = result {
        println!("空生日验证失败(符合预期): {}", e);
    }
}

#[test]
fn username_too_long_should_fail() {
    let user = User {
        username: "a".repeat(30),
        age: "25".to_string(),
        birthdate: "2000-01-01".to_string(),
    };
    let result = user.validate();
    assert!(result.is_err(), "用户名过长应该验证失败");
    if let Err(e) = result {
        println!("用户名过长验证失败(符合预期): {}", e);
    }
}

#[test]
fn age_non_numeric_should_fail() {
    let user = User {
        username: "validuser".to_string(),
        age: "abc".to_string(),
        birthdate: "2000-01-01".to_string(),
    };
    let result = user.validate();
    assert!(result.is_err(), "非数字年龄应该验证失败");
    if let Err(e) = result {
        println!("非数字年龄验证失败(符合预期): {}", e);
    }
}

#[test]
fn birthdate_wrong_format_should_fail() {
    let user = User {
        username: "validuser".to_string(),
        age: "25".to_string(),
        birthdate: "01-01-2000".to_string(),
    };
    let result = user.validate();
    assert!(result.is_err(), "错误格式生日应该验证失败");
    if let Err(e) = result {
        println!("错误格式生日验证失败(符合预期): {}", e);
    }
}
