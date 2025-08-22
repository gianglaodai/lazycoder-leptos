use std::borrow::Cow;

pub trait ToClass {
    fn to_class(self, out: &mut Vec<String>);
}

impl ToClass for &str {
    fn to_class(self, out: &mut Vec<String>) {
        let s = self.trim();
        if !s.is_empty() {
            out.push(s.to_string());
        }
    }
}

impl ToClass for String {
    fn to_class(self, out: &mut Vec<String>) {
        let s = self.trim().to_string();
        if !s.is_empty() {
            out.push(s);
        }
    }
}

impl ToClass for &String {
    fn to_class(self, out: &mut Vec<String>) {
        self.as_str().to_class(out);
    }
}

impl<'a> ToClass for Cow<'a, str> {
    fn to_class(self, out: &mut Vec<String>) {
        let s = self.trim();
        if !s.is_empty() {
            out.push(s.to_string());
        }
    }
}

impl<T: ToClass> ToClass for Option<T> {
    fn to_class(self, out: &mut Vec<String>) {
        if let Some(v) = self {
            v.to_class(out);
        }
    }
}

impl<'a, T: ToClass + Clone> ToClass for &'a Option<T> {
    fn to_class(self, out: &mut Vec<String>) {
        if let Some(v) = self {
            v.clone().to_class(out);
        }
    }
}

impl<T: ToClass> ToClass for Vec<T> {
    fn to_class(self, out: &mut Vec<String>) {
        for v in self {
            v.to_class(out);
        }
    }
}

impl<'a, T: ToClass + Clone> ToClass for &'a [T] {
    fn to_class(self, out: &mut Vec<String>) {
        for v in self {
            v.clone().to_class(out);
        }
    }
}

impl<T: ToClass, const N: usize> ToClass for [T; N] {
    fn to_class(self, out: &mut Vec<String>) {
        for v in self {
            v.to_class(out);
        }
    }
}

impl<T: ToClass> ToClass for (bool, T) {
    fn to_class(self, out: &mut Vec<String>) {
        let (cond, val) = self;
        if cond {
            val.to_class(out);
        }
    }
}

impl<T: ToClass> ToClass for (T, bool) {
    fn to_class(self, out: &mut Vec<String>) {
        let (val, cond) = self;
        if cond {
            val.to_class(out);
        }
    }
}

#[macro_export]
macro_rules! clsx {
    ( $( $x:expr ),* $(,)? ) => {{
        let mut out: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        $( $crate::utils::clsx::ToClass::to_class($x, &mut out); )*
        out.join(" ")
    }};
}
