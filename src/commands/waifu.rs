use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

#[derive(Serialize, Deserialize)]
struct Txt2ImgRequest {
    pub prompt: String,
    pub seed: i64,
    pub tiling: bool,
    pub cfg_scale: i64,
    pub steps: i64,
    pub sampler_name: String,
    pub negative_prompt: String,
    pub override_settings: Option<OverrideSettings>,
    pub override_settings_restore_after: bool,
}

#[derive(Serialize, Deserialize)]
struct OverrideSettings {
    pub sd_model_checkpoint: String,
}

#[derive(Serialize, Deserialize)]
struct Txt2ImgResponse {
    pub images: Vec<String>,
}

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("waifu")
        .description("txt2img using everything-v3.0 model")
        .create_option(|option| {
            option
                .name("prompt")
                .description("Image description")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("negative_prompt")
                .description("Negative prompt")
                .kind(CommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("seed")
                .description("Seed value")
                .kind(CommandOptionType::Integer)
        })
}

pub async fn run(options: &[CommandDataOption]) -> Result<(Vec<u8>, i64), String> {
    let prompt = match options
        .iter()
        .find(|o| o.name == "prompt")
        .expect("Expected prompt option")
        .resolved
        .as_ref()
        .expect("Expected prompt object") {
            CommandDataOptionValue::String(s) => Ok(s),
            _ => Err("Prompt was not a string"),
        }?.to_string();

    let negative_prompt = match options
        .iter()
        .find(|o| o.name == "negative_prompt") {
            Some(s) => {
                if let Some(CommandDataOptionValue::String(prompt)) = &s.resolved {
                    Ok(prompt.to_string())
                } else {
                    Err("")
                }
            }
            None => Ok("".to_string()),
        }?;


    let seed = match options
        .iter()
        .find(|o| o.name == "seed") {
            Some(s) => {
                if let Some(CommandDataOptionValue::Integer(seed)) = s.resolved {
                    Ok(seed)
                } else {
                    Err("")
                }
            }
            None => match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => Ok(n.as_secs() as i64),
                _ => Ok(0i64),
            }
        }?;

    let req_object = json!(Txt2ImgRequest {
        prompt,
        seed,
        negative_prompt,
        tiling: false,
        cfg_scale: 12,
        steps: 50,
        sampler_name: "DDIM".to_string(),
        override_settings: Some(OverrideSettings {
            sd_model_checkpoint: "Anything-V3.0-pruned.ckpt [2700c435]".to_string(),
        }),
        override_settings_restore_after: false,
    });

    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:7860/sdapi/v1/txt2img")
        .json(&req_object)
        .send()
        .await
        .map_err(|_| "Hmm".to_string())?;

    let resp = resp.json::<Txt2ImgResponse>()
        .await
        .map_err(|_| "Could not parse txt2img JSON")?;

    let image = resp.images
        .get(0)
        .ok_or("txt2img did not produce an image")?;

    let img = base64::decode(image)
        .map_err(|_| "Unable to decode txt2img base64 response".to_string())?;

    Ok((img, seed))
}
