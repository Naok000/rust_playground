#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind};

    #[allow(dead_code)]
    async fn my_async_call(url: &str) -> Result<serde_json::Value, Error> {
        // ? - ResultかOptionを返す非同期関数でのみ使用できる演算子
        // let response: serde_json::Value = reqwest::get(url)
        //     .await?
        //     .json::<serde_json::Value>()
        //     .await?;

        // Ok(response)

        let response = reqwest::get(url)
            .await
            .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response"))?;

        let json_response: serde_json::Value = response
            .json::<serde_json::Value>()
            .await
            .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to JSON"))?;

        Ok(json_response)
    }

    #[tokio::test]
    async fn tests_calls_async_fn() {
        let api_url: &str = "https://jsonplaceholder.typicode.com/users";
        let my_res: Result<serde_json::Value, std::io::Error> = my_async_call(api_url).await;

        match my_res {
            Ok(r) => {
                dbg!(r)
            }
            Err(_) => {
                panic!("Faild to make request!")
            }
        };
    }
}
