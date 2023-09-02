#[macro_export]
macro_rules! bitflags_union {
    ($ty:ty [$flag:ident]) => {
        <$ty>::$flag
    };
    ($ty:ty [$($flag:ident),+]) => {
        <$ty>::from_bits_truncate(0 $( | <$ty>::$flag.bits() )+)
    };
}

#[macro_export]
macro_rules! oneline_dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(oneline_dbg!($val)),+,)
    };
}

#[macro_export]
macro_rules! for_input {
    ($iter:ident, |$ele:ident| $body:tt) => {
        let mut m_iter = $iter;
        while let Some($ele) = Iterator::next(&mut m_iter) {
            let $ele = $ele?;
            $body;
        }
    };
}