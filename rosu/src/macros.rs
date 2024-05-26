
#[macro_export]
macro_rules! jni_fn {
    ($name:ident ($($arg_name:ident: $arg_type:ty),*):$reg_type:ty $body:block) => {
        #[no_mangle]
        pub extern "system" fn Java_rosu_Native_$name<'local>($(mut $arg_name: $arg_type),*) -> $reg_type$body
    };
}