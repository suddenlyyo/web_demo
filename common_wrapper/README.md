# Common Wrapper

通用响应封装库，提供统一的API响应格式。

## 功能特性

- 统一的响应数据结构
- 支持多种响应类型（单数据、列表、分页等）
- 标准化的错误处理
- 易于序列化和反序列化的结构

## 设计理念

### 1. 统一响应格式
在Web API开发中，统一的响应格式对于前端处理非常重要。本库提供了一套标准的响应结构，包括：
- 统一的状态码
- 统一的消息描述
- 标准化的数据结构

这样可以：
- 降低前端处理不同API响应的复杂度
- 提高API的可预测性和易用性
- 便于API文档的标准化

### 2. 类型安全
利用Rust的类型系统确保响应结构的正确性：
- 通过泛型支持不同类型的数据
- 编译时检查确保类型安全
- 明确的错误类型定义

### 3. 灵活性与专用性结合
提供多种专用的响应包装器，同时保持足够的灵活性：
- **SingleWrapper**：适用于单个数据的响应
- **ListWrapper**：适用于列表数据的响应
- **PageWrapper**：适用于分页数据的响应
- **ResponseWrapper**：通用响应包装器，支持自定义

这种设计既满足了常见场景的需求，又保持了处理特殊场景的灵活性。

### 4. 易于使用
提供简单直观的API：
- 一致的构造方法
- 链式调用支持
- 清晰的命名规范

### 5. 可序列化
所有响应结构都支持序列化，便于在Web API中使用：
- 实现了Serialize和Deserialize trait
- 与主流的JSON库兼容
- 支持自定义序列化配置

## 主要组件

### SingleWrapper
用于包装单个数据的响应结构：

```rust
use common_wrapper::SingleWrapper;

let user = User { name: "张三".to_string() };
let response = SingleWrapper::new();
response.set_success(user);
```

### ListWrapper
用于包装列表数据的响应结构：

```rust
use common_wrapper::ListWrapper;

let users = vec![user1, user2, user3];
let response = ListWrapper::new();
response.set_success(users);
```

### PageWrapper
用于包装分页数据的响应结构：

```rust
use common_wrapper::PageWrapper;

let page_data = PageWrapper::new();
page_data.set_success(users, total, page, size);
```

### ResponseWrapper
通用响应包装器，支持自定义响应结构：

```rust
use common_wrapper::ResponseWrapper;

let response = ResponseWrapper::success_default();
let error_response = ResponseWrapper::fail_default();
```

## 响应结构

所有响应都遵循统一的结构：

```json
{
  "code": 1,
  "message": "Success"
}
```

其中：
- `code`: 响应码，1表示成功，-1表示失败，-2表示未知错误
- `message`: 响应消息，描述操作结果

对于包含数据的响应（如SingleWrapper、ListWrapper等），还会有额外的`data`字段：

```json
{
  "code": 1,
  "message": "Success",
  "data": {}
}
```

## 错误处理

提供标准的错误类型枚举和错误处理机制：

```rust
use common_wrapper::WrapperErrEnum;

let error = WrapperErrEnum::UnknownError;
```

支持的错误类型包括：
- `Success`: 成功 (值为1)
- `Fail`: 失败 (值为-1)
- `UnknownError`: 未知错误 (值为-2)

## 使用示例

```rust
use common_wrapper::{SingleWrapper, ListWrapper, PageWrapper};

// 单个数据响应
let user = User::new("张三");
let mut response = SingleWrapper::new();
response.set_success(user);

// 列表响应
let users = vec![user1, user2];
let list_response = ListWrapper::success(users);

// 分页响应
let page_response = PageWrapper::success(users, 100, 1, 10);
```