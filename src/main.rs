use anyhow::Ok;
use async_openai::types::{
    ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use async_openai::Client;

    // Create client
    let client = Client::new().with_api_key("my api key"); // paste api key here

    // Create request using builder pattern
    // Every request struct has companion builder struct with same name + Args suffix
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content("使用JAVA语言求解leetcode第5题") // put your prompt here
            .build()?])
        // .max_tokens(200_u16)
        .max_tokens(1000u16) // change this to change the length of the response
        .build()?;

    // Call API
    let response = client
        .chat() // Get the API "group" (completions, images, etc.) from the client
        .create(request) // Make the API call in that "group"
        .await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {}",
            choice.index, choice.message.role, choice.message.content
        );
    }
    Ok(())
}
