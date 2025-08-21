pub trait ToClass {
    fn to_class(self, out: &mut Vec<String>);
}

impl ToClass for &str {
    fn to_class(self, out: &mut Vec<String>) {
        if !self.trim().is_empty() {
            out.push(self.to_string());
        }
    }
}

impl ToClass for String {
    fn to_class(self, out: &mut Vec<String>) {
        if !self.trim().is_empty() {
            out.push(self);
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

impl ToClass for (bool, &str) {
    fn to_class(self, out: &mut Vec<String>) {
        let (cond, s) = self;
        if cond {
            s.to_class(out);
        }
    }
}

impl ToClass for (bool, String) {
    fn to_class(self, out: &mut Vec<String>) {
        let (cond, s) = self;
        if cond {
            s.to_class(out);
        }
    }
}

#[macro_export]
macro_rules! clsx {
    ( $( $x:expr ),* $(,)? ) => {{
        let mut out = Vec::new();
        $( $crate::clsx::ToClass::to_class($x, &mut out); )*
        out.join(" ")
    }};
}
