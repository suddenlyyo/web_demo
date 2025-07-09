use crate::{
    DateTimeFormatEnum, ParameterValidator, Validatable, ValidateRulesEnum, ValidationErrorEnum,
    ValidationRule,
};

// ====================== 基本结构体验证 ======================
#[derive(Debug, Validatable)]
struct BasicUser {
    #[validate(NotNone, Length, desc = "用户名", length = "3~20")]
    username: String,

    #[validate(
        NotNone,
        NumberMin,
        NumberMax,
        desc = "年龄",
        number_min = 1,
        number_max = 120
    )]
    age: i32,

    #[validate(Date, desc = "生日", date_format = "Year")]
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
    assert_eq!(
        invalid_username.validate(),
        Err(ValidationErrorEnum::Length(
            "用户名".to_string(),
            "长度必须在 3~20 之间".to_string()
        ))
    );

    // 年龄超出范围
    let invalid_age = BasicUser {
        username: "john_doe".to_string(),
        age: 150, // 超出最大值
        birthdate: "1990-01-01".to_string(),
    };
    assert_eq!(
        invalid_age.validate(),
        Err(ValidationErrorEnum::NumberMax("年龄".to_string(), 120))
    );

    // 日期格式错误
    let invalid_date = BasicUser {
        username: "john_doe".to_string(),
        age: 30,
        birthdate: "1990/01/01".to_string(), // 格式错误
    };
    assert_eq!(
        invalid_date.validate(),
        Err(ValidationErrorEnum::Format("生日".to_string()))
    );
}

// ====================== 嵌套结构体验证 ======================
#[derive(Debug, Validatable)]
struct Address {
    #[validate(NotNone, Length, desc = "街道", length = "5~50")]
    street: String,

    #[validate(NotNone, desc = "城市")]
    city: String,

    #[validate(Length, desc = "邮编", length = "5~10")]
    zipcode: Option<String>,
}

#[derive(Debug, Validatable)]
struct UserProfile {
    #[validate(Structure, desc = "基础信息")]
    basic: BasicUser,

    #[validate(Structure, desc = "地址")]
    address: Address,

    #[validate(Structure, desc = "备用地址")]
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
    assert_eq!(
        invalid_basic.validate(),
        Err(ValidationErrorEnum::Length(
            "用户名".to_string(),
            "长度必须在 3~20 之间".to_string()
        ))
    );

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
    assert_eq!(
        invalid_address.validate(),
        Err(ValidationErrorEnum::Length(
            "街道".to_string(),
            "长度必须在 5~50 之间".to_string()
        ))
    );

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
    assert_eq!(
        invalid_secondary.validate(),
        Err(ValidationErrorEnum::NotNone("城市".to_string()))
    );
}

// ====================== 枚举验证 ======================
#[derive(Debug, Validatable)]
enum PaymentMethod {
    #[validate(desc = "信用卡支付")]
    CreditCard {
        #[validate(NotNone, Length, desc = "卡号", length = "16")]
        number: String,

        #[validate(Date, desc = "过期日期", date_format = "YearNoSplit")]
        expiry: String,

        #[validate(NotNone, Length, desc = "安全码", length = "3")]
        cvv: String,
    },

    #[validate(desc = "PayPal支付")]
    PayPal {
        #[validate(NotNone, desc = "PayPal邮箱")]
        email: String,
    },

    #[validate(desc = "银行转账")]
    BankTransfer {
        #[validate(NotNone, Length, desc = "账号", length = "10~20")]
        account: String,

        #[validate(NotNone, Length, desc = "路由号", length = "9")]
        routing: String,
    },
}

#[test]
fn test_enum_validation() {
    // 有效的信用卡支付
    let valid_credit = PaymentMethod::CreditCard {
        number: "4111111111111111".to_string(),
        expiry: "202512".to_string(),
        cvv: "123".to_string(),
    };
    assert!(valid_credit.validate().is_ok());

    // 无效的信用卡支付 - 卡号错误
    let invalid_credit_number = PaymentMethod::CreditCard {
        number: "1234".to_string(), // 太短
        expiry: "202512".to_string(),
        cvv: "123".to_string(),
    };
    assert_eq!(
        invalid_credit_number.validate(),
        Err(ValidationErrorEnum::Length(
            "卡号".to_string(),
            "长度必须为 16".to_string()
        ))
    );

    // 无效的信用卡支付 - 过期日期格式错误
    let invalid_credit_expiry = PaymentMethod::CreditCard {
        number: "4111111111111111".to_string(),
        expiry: "2025-12".to_string(), // 格式错误
        cvv: "123".to_string(),
    };
    assert_eq!(
        invalid_credit_expiry.validate(),
        Err(ValidationErrorEnum::Format("过期日期".to_string()))
    );

    // 有效的PayPal支付
    let valid_paypal = PaymentMethod::PayPal {
        email: "user@example.com".to_string(),
    };
    assert!(valid_paypal.validate().is_ok());

    // 无效的PayPal支付 - 邮箱为空
    let invalid_paypal = PaymentMethod::PayPal {
        email: "".to_string(), // 不能为空
    };
    assert_eq!(
        invalid_paypal.validate(),
        Err(ValidationErrorEnum::NotNone("PayPal邮箱".to_string()))
    );

    // 有效的银行转账
    let valid_bank = PaymentMethod::BankTransfer {
        account: "1234567890".to_string(),
        routing: "123456789".to_string(),
    };
    assert!(valid_bank.validate().is_ok());

    // 无效的银行转账 - 账号太短
    let invalid_bank_account = PaymentMethod::BankTransfer {
        account: "123".to_string(), // 太短
        routing: "123456789".to_string(),
    };
    assert_eq!(
        invalid_bank_account.validate(),
        Err(ValidationErrorEnum::Length(
            "账号".to_string(),
            "长度必须在 10~20 之间".to_string()
        ))
    );
}

// ====================== 嵌套枚举验证 ======================
#[derive(Debug, Validatable)]
enum OrderStatus {
    #[validate(desc = "待支付")]
    Pending {
        #[validate(NotNone, desc = "创建时间")]
        created_at: String,
    },

    #[validate(desc = "已支付")]
    Paid {
        #[validate(Structure, desc = "支付方式")]
        payment_method: PaymentMethod,

        #[validate(NotNone, desc = "支付时间")]
        paid_at: String,
    },

    #[validate(desc = "已发货")]
    Shipped {
        #[validate(NotNone, desc = "发货时间")]
        shipped_at: String,

        #[validate(NotNone, Length, desc = "物流单号", length = "12")]
        tracking_number: String,
    },
}

#[derive(Debug, Validatable)]
struct Order {
    #[validate(NotNone, desc = "订单ID")]
    id: String,

    #[validate(Structure, desc = "订单状态")]
    status: OrderStatus,
}

#[test]
fn test_nested_enum_validation() {
    // 有效的待支付订单
    let valid_pending = Order {
        id: "ORD-12345".to_string(),
        status: OrderStatus::Pending {
            created_at: "2023-10-01 10:00:00".to_string(),
        },
    };
    assert!(valid_pending.validate().is_ok());

    // 无效的待支付订单 - 创建时间为空
    let invalid_pending = Order {
        id: "ORD-12345".to_string(),
        status: OrderStatus::Pending {
            created_at: "".to_string(), // 不能为空
        },
    };
    assert_eq!(
        invalid_pending.validate(),
        Err(ValidationErrorEnum::NotNone("创建时间".to_string()))
    );

    // 有效的已支付订单
    let valid_paid = Order {
        id: "ORD-67890".to_string(),
        status: OrderStatus::Paid {
            payment_method: PaymentMethod::PayPal {
                email: "user@example.com".to_string(),
            },
            paid_at: "2023-10-01 11:30:00".to_string(),
        },
    };
    assert!(valid_paid.validate().is_ok());

    // 无效的已支付订单 - 支付方式无效
    let invalid_paid = Order {
        id: "ORD-67890".to_string(),
        status: OrderStatus::Paid {
            payment_method: PaymentMethod::PayPal {
                email: "".to_string(), // 邮箱不能为空
            },
            paid_at: "2023-10-01 11:30:00".to_string(),
        },
    };
    assert_eq!(
        invalid_paid.validate(),
        Err(ValidationErrorEnum::NotNone("PayPal邮箱".to_string()))
    );

    // 有效的已发货订单
    let valid_shipped = Order {
        id: "ORD-54321".to_string(),
        status: OrderStatus::Shipped {
            shipped_at: "2023-10-02 09:15:00".to_string(),
            tracking_number: "TRACK123456".to_string(), // 12字符
        },
    };
    assert!(valid_shipped.validate().is_ok());

    // 无效的已发货订单 - 物流单号错误
    let invalid_shipped = Order {
        id: "ORD-54321".to_string(),
        status: OrderStatus::Shipped {
            shipped_at: "2023-10-02 09:15:00".to_string(),
            tracking_number: "TRACK123".to_string(), // 太短
        },
    };
    assert_eq!(
        invalid_shipped.validate(),
        Err(ValidationErrorEnum::Length(
            "物流单号".to_string(),
            "长度必须为 12".to_string()
        ))
    );
}

// ====================== 复杂嵌套验证 ======================
#[derive(Debug, Validatable)]
struct OrderItem {
    #[validate(NotNone, desc = "产品ID")]
    product_id: String,

    #[validate(NumberMin, desc = "数量", number_min = 1)]
    quantity: i32,
}

#[derive(Debug, Validatable)]
struct CompleteOrder {
    #[validate(Structure, desc = "订单信息")]
    order: Order,

    #[validate(Structure, desc = "订单项")]
    items: Vec<OrderItem>,

    #[validate(Structure, desc = "配送地址")]
    shipping_address: Address,
}

#[test]
fn test_complex_nested_validation() {
    // 有效的完整订单
    let valid_order = CompleteOrder {
        order: Order {
            id: "ORD-99999".to_string(),
            status: OrderStatus::Shipped {
                shipped_at: "2023-10-03 14:20:00".to_string(),
                tracking_number: "SHIP12345678".to_string(),
            },
        },
        items: vec![
            OrderItem {
                product_id: "PROD-100".to_string(),
                quantity: 2,
            },
            OrderItem {
                product_id: "PROD-200".to_string(),
                quantity: 1,
            },
        ],
        shipping_address: Address {
            street: "789 Pine Road".to_string(),
            city: "Gotham".to_string(),
            zipcode: Some("67890".to_string()),
        },
    };
    assert!(valid_order.validate().is_ok());

    // 订单项数量为0
    let invalid_item_quantity = CompleteOrder {
        order: Order {
            id: "ORD-99999".to_string(),
            status: OrderStatus::Shipped {
                shipped_at: "2023-10-03 14:20:00".to_string(),
                tracking_number: "SHIP12345678".to_string(),
            },
        },
        items: vec![OrderItem {
            product_id: "PROD-100".to_string(),
            quantity: 0, // 最小值1
        }],
        shipping_address: Address {
            street: "789 Pine Road".to_string(),
            city: "Gotham".to_string(),
            zipcode: Some("67890".to_string()),
        },
    };
    assert_eq!(
        invalid_item_quantity.validate(),
        Err(ValidationErrorEnum::NumberMin("数量".to_string(), 1))
    );

    // 订单状态无效
    let invalid_order_status = CompleteOrder {
        order: Order {
            id: "ORD-99999".to_string(),
            status: OrderStatus::Paid {
                payment_method: PaymentMethod::CreditCard {
                    number: "4111111111111111".to_string(),
                    expiry: "2025-12".to_string(), // 格式错误 (应该是202512)
                    cvv: "123".to_string(),
                },
                paid_at: "2023-10-03 12:00:00".to_string(),
            },
        },
        items: vec![OrderItem {
            product_id: "PROD-100".to_string(),
            quantity: 2,
        }],
        shipping_address: Address {
            street: "789 Pine Road".to_string(),
            city: "Gotham".to_string(),
            zipcode: Some("67890".to_string()),
        },
    };
    assert_eq!(
        invalid_order_status.validate(),
        Err(ValidationErrorEnum::Format("过期日期".to_string()))
    );

    // 配送地址无效
    let invalid_address = CompleteOrder {
        order: Order {
            id: "ORD-99999".to_string(),
            status: OrderStatus::Shipped {
                shipped_at: "2023-10-03 14:20:00".to_string(),
                tracking_number: "SHIP12345678".to_string(),
            },
        },
        items: vec![OrderItem {
            product_id: "PROD-100".to_string(),
            quantity: 2,
        }],
        shipping_address: Address {
            street: "789 Pine".to_string(), // 太短
            city: "Gotham".to_string(),
            zipcode: Some("67890".to_string()),
        },
    };
    assert_eq!(
        invalid_address.validate(),
        Err(ValidationErrorEnum::Length(
            "街道".to_string(),
            "长度必须在 5~50 之间".to_string()
        ))
    );
}
