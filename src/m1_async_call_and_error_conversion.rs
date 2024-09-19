use std::io::{Error, ErrorKind};

async fn my_async_call(url: &str) -> Result<serde_json::Value, std::io::Error> {
    // Fetch response from the provided URL
    let response = reqwest::get(url)
        .await
        // Convert reqwest Error into a Generic Error
        .map_err(|e| Error::new(ErrorKind::Other, format!("could not retrieve response: {}", e)))?;

    // Parse the response body as JSON
    let json_body = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("could not retrieve json: {}", e)))?;

    // Return the parsed JSON body
    Ok(json_body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_call() {
        // Test URL for a simple JSON placeholder endpoint
        let url = "https://jsonplaceholder.typicode.com/todos/1";

        // Call the async function and handle the result
        match my_async_call(url).await {
            Ok(json) => {
                // Print the JSON result and assert key values for test validation
                dbg!(&json);
                assert!(json["id"].is_number(), "Expected 'id' to be a number");
            }
            Err(e) => {
                // Print error if the call fails
                dbg!("Error: {}", e);
                assert!(false, "The async call failed");
            }
        }
    }
}
