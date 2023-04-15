pub fn test1() {
    // Let's chat! Initial setup
    let input = "You are connected to an API that will receive your response and treat it as its request.
    This API will then return a response that you can use to know the result of your request.
    This is a simple API that will give you some storage capabilities, you can store some text and retrieve it later.
    Your response needs to be in JSON and have the following structure:
    {
        \"command\": SAVE | GET,
        \"data\": string,
    }
    The command will depend on what you want to do, and the data parameter will depend
    on the command you want to execute. For example, if you want to save a text, you
    will need to provide the text you want to save as the data parameter. If you want to
    retrieve a text, you will need to provide the id of the text you want to retrieve
    as the data parameter.
    ".to_string();

    let response = gpt.completion(input.clone()).await?;
    println!("Response from GPT: {}", response.to_string());
    gpt.context.push(response);

    // Response to GPT
    let input = "Response from the API:
    {
        \"status\": \"success\",
        \"id\": \"1\"
    }

    You can now send a new request to the API.
    "
    .to_string();
    let response = gpt.completion(input.clone()).await?;
    println!("Response from GPT: {}", response);
    gpt.context.push(response);

    // Response to GPT
    let input = "Response from the API:
    {
        \"status\": \"success\",
        \"id\": \"1\",
        \"data\": \"This is a sample text to save into storage.\"
    }

    You can now send a new request to the API.
    "
    .to_string();
    let response = gpt.completion(input.clone()).await?;
    println!("Response from GPT: {}", response);
    gpt.context.push(response);

    println!("Context: {:?}", gpt.context);
}
