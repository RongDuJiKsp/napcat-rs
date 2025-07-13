use anyhow::anyhow;
use kovi::utils::load_json_data;
use kovi::RuntimeBot;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::any::type_name;
use std::sync::OnceLock;

pub async fn init_config<T: Default + Serialize + DeserializeOwned>(
    runtime_bot: &RuntimeBot,
    config_name: &str,
    target: &OnceLock<T>,
) -> Result<(), anyhow::Error> {
    let config = load_json_data(T::default(), runtime_bot.get_data_path().join(config_name))
        .map_err(|e| anyhow!("Error loading command config: {}", e))?;
    target
        .set(config)
        .map_err(|_e| anyhow!("初始化{}时出现重复设置", type_name::<T>()))?;
    Ok(())
}
