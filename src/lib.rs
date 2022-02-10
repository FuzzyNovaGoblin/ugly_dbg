#[macro_export]
macro_rules! dbgu {
    (# $($x:expr),*) => {
        dbgu!(internal false, $($x)*)
    };

    ($($x:expr),*) => {
        dbgu!(internal true, $($x)*)
    };

    (internal $p:expr, $($x:expr),*) => {
        $(
            let dbg_str = if $p {format!("{:?}", $x)}else {format!("{:#?}", $x)};

            eprint!("[{}:{}]",  file!(), line!());

            if dbg_str.contains("\n"){
                eprintln!(":\n{}\nend_of:[{}:{}]", dbg_str,file!(), line!())
            }
            else{
                eprintln!(" {} = {}",stringify!($x), dbg_str)
            }
        )*
    };
}
