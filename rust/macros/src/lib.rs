#[macro_export]
macro_rules! hashmap {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($k, $v);
            )*
            hm
        }
    };
    ( $( $k:expr => $v:expr, )+ ) => {
        macros::hashmap!($($k => $v),+)
    };
}
