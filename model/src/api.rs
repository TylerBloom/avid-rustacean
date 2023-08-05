use std::fmt::Display;

use crate::extend;

/* ------ Base APIs ------ */
const BASE_API: Url<0> = Url::from("api/v1");

/* ------ Blog post APIs ------ */
const BLOG_ENDPOINT: Url<0> = Url::from("/blog");
pub const BLOG_API: Url<0> = extend!(BASE_API, BLOG_ENDPOINT);

const BLOG_POST_ENDPOINT: Url<1> = Url::new("/:p_id", [":p_id"]);
pub const BLOG_POST_API: Url<1> = extend!(BLOG_API, BLOG_POST_ENDPOINT);

/* ------ URL definition ------ */

#[macro_export]
macro_rules! extend {
    ($url:ident, $ext:literal) => {
        Url::new(const_format::concatcp!($url.route, $ext), $url.replacements)
    };
    ($url:ident, $ext:literal, $reps:literal) => {
        Url::new(
            const_format::concatcp!($url.route, $ext),
            array_concat::concat_arrays!($url.replacements, $reps),
        )
    };
    ($url:ident, $ext:ident) => {{
        const A: [&str; $url.replacements.len()] = $url.replacements;
        const B: [&str; $ext.replacements.len()] = $ext.replacements;
        Url::new(
            const_format::concatcp!($url.route, $ext.route),
            array_concat::concat_arrays!(A, B),
        )
    }};
    ($url:ident, $ext:ident, $reps:literal) => {
        const A: [&str; $url.replacements.len()] = $url.replacements;
        const B: [&str; $ext.replacements.len()] = $ext.replacements;
        Url::new(
            const_format::concatcp!($url.route, $ext.route),
            array_concat::concat_arrays!(A, B, $reps),
        )
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Url<const N: usize> {
    pub(crate) route: &'static str,
    pub(crate) replacements: [&'static str; N],
}

impl Url<0> {
    pub const fn from(route: &'static str) -> Self {
        Self {
            route,
            replacements: [],
        }
    }
}

impl<const N: usize> Url<N> {
    pub const fn new(route: &'static str, replacements: [&'static str; N]) -> Self {
        Self {
            route,
            replacements,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        self.route
    }

    pub fn replace(&self, values: [&str; N]) -> String {
        let mut digest = self.route.to_string();
        for (pattern, value) in self.replacements.iter().zip(values.iter()) {
            digest = digest.replacen(pattern, value, 1);
        }
        digest
    }
}

impl Url<1> {
    pub fn insert<S: ToString>(&self, value: S) -> String {
        self.route.replace(self.replacements[0], &value.to_string())
    }
}

impl<const N: usize> Display for Url<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.route)
    }
}
