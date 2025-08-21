use crate::utils::clsx::ToClass;
use crate::utils::tw::tw_merge;

#[macro_export]
macro_rules! cn {
    ( $( $x:expr ),* $(,)? ) => {{
        let s = $crate::clsx![ $( $x ),* ];
        $crate::tw_merge::tw_merge([s])
    }};
}

pub fn cn_iter<I, T>(inputs: I) -> String
where
    I: IntoIterator<Item = T>,
    T: ToClass,
{
    let mut tmp = Vec::new();
    for v in inputs {
        v.to_class(&mut tmp);
    }
    tw_merge(tmp)
}
