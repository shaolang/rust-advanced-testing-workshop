//! Write a custom `is_redirect` matcher that checks if a `StatusCode` is a redirect.
use googletest::{matcher::{Matcher, MatcherResult}, prelude::MatcherBase};
use http::StatusCode;

pub fn is_redirect() -> impl Matcher<StatusCode> {
    StatusCodeMatcher {
        expected: StatusCode::MOVED_PERMANENTLY,
    }
}

#[derive(MatcherBase)]
struct StatusCodeMatcher {
    expected: StatusCode,
}

impl Matcher<StatusCode> for StatusCodeMatcher {
    fn matches(&self, actual: StatusCode) -> googletest::matcher::MatcherResult {
        if self.expected == actual {
            MatcherResult::Match
        } else {
            MatcherResult::NoMatch
        }
    }

    fn describe(&self, matcher_result: googletest::matcher::MatcherResult) -> googletest::description::Description {
        match matcher_result {
            MatcherResult::Match => format!("is a redirection status code").into(),
            MatcherResult::NoMatch => format!("isn't a redirection status code").into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::is_redirect;
    use googletest::assert_that;
    use http::StatusCode;

    #[test]
    fn success() {
        assert_that!(StatusCode::MOVED_PERMANENTLY, is_redirect());
    }

    #[test]
    fn failure() {
        assert_that!(StatusCode::OK, is_redirect());
    }
}
