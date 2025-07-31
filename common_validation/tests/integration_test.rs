use common_validation::{DateTimeFormatEnum, ParameterValidator, Validatable, ValidationErrorEnum, ValidationRule, ValidationRulesEnum};

#[derive(Debug)]
struct User {
    username: String,
    age: String,
    birthdate: String,
}

impl Validatable for User {
    fn validate(&self) -> Result<(), ValidationErrorEnum> {
        // 用户名验证规则
        let username_rule = ValidationRule::new("用户名")
            .with_rule(ValidationRulesEnum::NotNone)
            .with_length("3~20");

        ParameterValidator::validate_value(&self.username, &username_rule)?;

        // 年龄验证规则
        let age_rule = ValidationRule::new("年龄")
            .with_rule(ValidationRulesEnum::NotNone)
            .with_rule(ValidationRulesEnum::NumberMin)
            .with_rule(ValidationRulesEnum::NumberMax)
            .with_number_range(Some(1), Some(120));

        ParameterValidator::validate_value(&self.age, &age_rule)?;

        // 生日验证规则
        let birthdate_rule = ValidationRule::new("生日")
            .with_rule(ValidationRulesEnum::Date)
            .with_date_format(Some(DateTimeFormatEnum::Year));

        ParameterValidator::validate_value(&self.birthdate, &birthdate_rule)?;

        Ok(())
    }
}

#[test]
fn validate_test() {
    // 打印输出测试命令: cargo test -p common_validation --test integration_test -- --show-output 
    // 创建测试用户
    let valid_user = User {
        username: "john_doe".to_string(),
        age: "30".to_string(),
        birthdate: "1990-01-011".to_string(),
    };

    let invalid_user = User {
        username: "jd".to_string(),          // 太短
        age: "150".to_string(),              // 超过最大值
        birthdate: "1990/01/01".to_string(), // 格式错误
    };

    // 验证有效用户
    match valid_user.validate() {
        Ok(_) => println!("有效用户验证通过"),
        Err(e) => println!("有效用户验证失败: {}", e),
    }

    // 验证无效用户
    match invalid_user.validate() {
        Ok(_) => println!("无效用户验证通过"),
        Err(e) => println!("无效用户验证失败: {}", e),
    }

    // 直接值验证示例
    let email_rule = ValidationRule::new("邮箱")
        .with_rule(ValidationRulesEnum::NotNone)
        .with_length("5~100");

    match ParameterValidator::validate_value("test@example.com", &email_rule) {
        Ok(_) => println!("邮箱验证通过"),
        Err(e) => println!("邮箱验证失败: {}", e),
    }
}
