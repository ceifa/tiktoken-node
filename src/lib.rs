use napi::bindgen_prelude::Error;
use napi_derive::napi;

#[napi]
pub struct Encoding {
    pub(crate) encoding: tiktoken_rs::CoreBPE,
}

#[napi]
impl Encoding {
    #[napi]
    pub fn encode(&self, text: String) -> Vec<u32> {
        self.encoding
            .encode_with_special_tokens(&text)
            .iter()
            .map(|x| *x as u32)
            .collect()
    }

    #[napi]
    pub fn decode(&self, tokens: Vec<u32>) -> Result<String, Error> {
        match self
            .encoding
            .decode(tokens.iter().map(|x| *x as usize).collect())
        {
            Ok(text) => Ok(text),
            Err(err) => Err(Error::from_reason(err.to_string())),
        }
    }

    // This is much slower than the sync version, have to test if it's worth it on larger batches
    // #[napi]
    // pub async fn encode_batch(&self, texts: Vec<String>) -> Vec<Vec<u32>> {
    //     let mut tasks: Vec<tokio::task::JoinHandle<Vec<u32>>> = Vec::new();
    //     for text in texts {
    //         let encoding = self.encoding.clone();
    //         tasks.push(tokio::spawn(async move {
    //             encoding
    //                 .encode_with_special_tokens(&text)
    //                 .iter()
    //                 .map(|x| *x as u32)
    //                 .collect()
    //         }));
    //     }

    //     futures::future::join_all(tasks)
    //         .await
    //         .iter()
    //         .map(|x| x.as_ref().unwrap().clone())
    //         .collect()
    // }
}

#[napi]
pub fn get_encoding(encoding: String) -> Result<Encoding, Error> {
    let encoding: Result<tiktoken_rs::CoreBPE, Error> = match encoding.as_str() {
        "gpt2" => Ok(tiktoken_rs::r50k_base().unwrap()),
        "r50k_base" => Ok(tiktoken_rs::r50k_base().unwrap()),
        "p50k_base" => Ok(tiktoken_rs::p50k_base().unwrap()),
        "p50k_edit" => Ok(tiktoken_rs::p50k_edit().unwrap()),
        "cl100k_base" => Ok(tiktoken_rs::cl100k_base().unwrap()),
        _ => Err(Error::from_reason("Invalid encoding")),
    };

    match encoding {
        Ok(encoding) => Ok(Encoding { encoding }),
        Err(err) => Err(err),
    }
}

#[napi]
pub fn encoding_for_model(model_name: String) -> Result<Encoding, Error> {
    let encoding = tiktoken_rs::get_bpe_from_model(&model_name);
    match encoding {
        Ok(encoding) => Ok(Encoding { encoding }),
        Err(err) => Err(Error::from_reason(err.to_string())),
    }
}
