# 使用Rust语言调用openai API解决leetcode第5题 回文数的程序

## 如何运行

1.在代码的第11行填入你的**openai API key**

```rust
    use async_openai::Client;

    // Create client
    let client = Client::new().with_api_key("my api key"); // paste api key here

    // Create request using builder pattern
    // Every request struct has companion builder struct with same name + Args suffix
    let request = CreateChatCompletionRequestArgs::default()
```

2.(可选) 在代码的第19行填入你的问题 **(默认是使用JAVA语言求解leetcode第5题)**\
可以根据情况更改代码第22行中的**max_tokens**中的数字，数字越大，结果越长

```rust
    // Create request using builder pattern
    // Every request struct has companion builder struct with same name + Args suffix
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content("使用JAVA语言求解leetcode第5题") // put your prompt here
            .build()?])
        // .max_tokens(200_u16)
        .max_tokens(1000u16)
        .build()?;
```

3.在终端中运行cargo run

## 代码说明

本代码使用[async-openai crate](https://docs.rs/async-openai/0.11.0/async_openai)。\
源代码来自[https://docs.rs/async-openai/0.11.0/async_openai/#making-requests]
和[https://github.com/64bit/async-openai/blob/main/examples/chat/src/main.rs]
