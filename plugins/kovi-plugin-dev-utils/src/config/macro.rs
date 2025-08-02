#[macro_export]
macro_rules! config {
    // 三个参数：显式传入文件名
    ($type:ident, $static_ref:ident, $filename:expr) => {
        impl $type {
            pub fn init(
                runtime_bot: &::kovi::RuntimeBot,
            ) -> ::core::result::Result<(), ::anyhow::Error> {
                ::kovi_plugin_dev_utils::config::initfn::init_config(
                    runtime_bot,
                    $filename,
                    &$static_ref,
                )
            }

            pub fn get() -> &'static $type {
                $static_ref.get().unwrap()
            }
        }
    };

    // 两个参数：使用默认文件名
    ($type:ident, $static_ref:ident) => {
        $crate::config!($type, $static_ref, concat!(stringify!($type), ".json"));
    };
}
#[macro_export]
macro_rules! data {
    ($type:ident, $static_name:ident, $file:expr) => {
        impl $type {
            pub fn init(runtime_bot: &::kovi::RuntimeBot) -> Result<(), ::anyhow::Error> {
                ::kovi_plugin_dev_utils::config::initfn::init_data(
                    runtime_bot,
                    $file,
                    &$static_name,
                )
            }

            pub fn get() -> ::std::sync::Arc<::kovi::tokio::sync::RwLock<Self>> {
                $static_name.get().unwrap().clone()
            }
        }
    };
    ($type:ident, $static_name:ident) => {
        $crate::impl_data_helper!($type, $static_name, concat!(stringify!($type), ".json"));
    };
}
