//! Contains numerical helper traits and functions

use crate::geometry::Size;
use crate::layout::AvailableSpace;

/// A trait to conveniently calculate minimums and maximums when some data may not be defined
///
/// If the left-hand value is [`None`], these operations return [`None`].
/// If the right-hand value is [`None`], it is treated as zero.
pub(crate) trait MaybeMath<In, Out> : Sized {
    /// Returns the minimum of `self` and `rhs`
    fn maybe_min(self, rhs: In) -> Out;
    fn maybe_floor_at(self, rhs: In) -> Out {
        self.maybe_min(rhs)
    }

    /// Returns the maximum of `self` and `rhs`
    fn maybe_max(self, rhs: In) -> Out;
    fn maybe_ceil_at(self, rhs: In) -> Out {
        self.maybe_max(rhs)
    }

    /// Adds `self` and `rhs`.
    fn maybe_add(self, rhs: In) -> Out;

    /// Subtracts rhs from `self`, treating [`None`] values as default
    fn maybe_sub(self, rhs: In) -> Out;
}

impl MaybeMath<Option<f32>, Option<f32>> for Option<f32> {
    fn maybe_min(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.min(r)),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_max(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.max(r)),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_add(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_sub(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l - r),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }
}

impl MaybeMath<f32, Option<f32>> for Option<f32> {
    fn maybe_min(self, rhs: f32) -> Option<f32> {
        self.map(|val| val.min(rhs))
    }

    fn maybe_max(self, rhs: f32) -> Option<f32> {
        self.map(|val| val.max(rhs))
    }

    fn maybe_add(self, rhs: f32) -> Option<f32> {
        self.map(|val| val + rhs)
    }

    fn maybe_sub(self, rhs: f32) -> Option<f32> {
        self.map(|val| val - rhs)
    }
}

impl MaybeMath<Option<f32>, f32> for f32 {
    fn maybe_min(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self.min(val),
            None => self,
        }
    }

    fn maybe_max(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self.max(val),
            None => self,
        }
    }

    fn maybe_add(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self + val,
            None => self,
        }
    }

    fn maybe_sub(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self - val,
            None => self,
        }
    }
}

impl MaybeMath<f32, AvailableSpace> for AvailableSpace {
    fn maybe_min(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.min(rhs)),
            AvailableSpace::MinContent => AvailableSpace::Definite(rhs),
            AvailableSpace::MaxContent => AvailableSpace::Definite(rhs),
        }
    }
    fn maybe_max(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.max(rhs)),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
    fn maybe_add(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val + rhs),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
    fn maybe_sub(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val - rhs),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

impl MaybeMath<Option<f32>, AvailableSpace> for AvailableSpace {
    fn maybe_min(self, rhs: Option<f32>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val.min(rhs)),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, Some(rhs)) => AvailableSpace::Definite(rhs),
            (AvailableSpace::MinContent, None) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, Some(rhs)) => AvailableSpace::Definite(rhs),
            (AvailableSpace::MaxContent, None) => AvailableSpace::MaxContent,
        }
    }
    fn maybe_max(self, rhs: Option<f32>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val.max(rhs)),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
    fn maybe_add(self, rhs: Option<f32>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val + rhs),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
    fn maybe_sub(self, rhs: Option<f32>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val - rhs),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
}


#[cfg(test)]
mod tests {
    mod lhs_option_f32_rhs_option_f32 {

        use crate::math::MaybeMath;
        use rstest::rstest;

        #[rstest]
        #[case(Some(3.0), Some(5.0), Some(3.0))]
        #[case(Some(5.0), Some(3.0), Some(3.0))]
        #[case(Some(3.0), None, Some(3.0))]
        #[case(None, Some(3.0), None)]
        #[case(None, None, None)]
        fn test_maybe_min(#[case] lhs: Option<f32>, #[case] rhs: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_min(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), Some(5.0), Some(5.0))]
        #[case(Some(5.0), Some(3.0), Some(5.0))]
        #[case(Some(3.0), None, Some(3.0))]
        #[case(None, Some(3.0), None)]
        #[case(None, None, None)]
        fn test_maybe_max(#[case] lhs: Option<f32>, #[case] rhs: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_max(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), Some(5.0), Some(8.0))]
        #[case(Some(5.0), Some(3.0), Some(8.0))]
        #[case(Some(3.0), None, Some(3.0))]
        #[case(None, Some(3.0), None)]
        #[case(None, None, None)]
        fn test_maybe_add(#[case] lhs: Option<f32>, #[case] rhs: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_add(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), Some(5.0), Some(-2.0))]
        #[case(Some(5.0), Some(3.0), Some(2.0))]
        #[case(Some(3.0), None, Some(3.0))]
        #[case(None, Some(3.0), None)]
        #[case(None, None, None)]
        fn test_maybe_sub(#[case] lhs: Option<f32>, #[case] rhs: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_sub(rhs), expected);
        }
    }

    mod lhs_option_f32_rhs_f32 {

        use crate::math::MaybeMath;
        use rstest::rstest;

        #[rstest]
        #[case(Some(3.0), 5.0, Some(3.0))]
        #[case(Some(5.0), 3.0, Some(3.0))]
        #[case(None, 3.0, None)]
        fn test_maybe_min(#[case] lhs: Option<f32>, #[case] rhs: f32, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_min(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), 5.0, Some(5.0))]
        #[case(Some(5.0), 3.0, Some(5.0))]
        #[case(None, 3.0, None)]
        fn test_maybe_max(#[case] lhs: Option<f32>, #[case] rhs: f32, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_max(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), 5.0, Some(8.0))]
        #[case(Some(5.0), 3.0, Some(8.0))]
        #[case(None, 3.0, None)]
        fn test_maybe_add(#[case] lhs: Option<f32>, #[case] rhs: f32, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_add(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), 5.0, Some(-2.0))]
        #[case(Some(5.0), 3.0, Some(2.0))]
        #[case(None, 3.0, None)]
        fn test_maybe_sub(#[case] lhs: Option<f32>, #[case] rhs: f32, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_sub(rhs), expected);
        }
    }

    mod lhs_f32_rhs_option_f32 {

        use crate::math::MaybeMath;
        use rstest::rstest;

        #[rstest]
        #[case(3.0, Some(5.0), 3.0)]
        #[case(5.0, Some(3.0), 3.0)]
        #[case(3.0, None, 3.0)]
        fn test_maybe_min(#[case] lhs: f32, #[case] rhs: Option<f32>, #[case] expected: f32) {
            assert_eq!(lhs.maybe_min(rhs), expected);
        }

        #[rstest]
        #[case(3.0, Some(5.0), 5.0)]
        #[case(5.0, Some(3.0), 5.0)]
        #[case(3.0, None, 3.0)]
        fn test_maybe_max(#[case] lhs: f32, #[case] rhs: Option<f32>, #[case] expected: f32) {
            assert_eq!(lhs.maybe_max(rhs), expected);
        }

        #[rstest]
        #[case(3.0, Some(5.0), 8.0)]
        #[case(5.0, Some(3.0), 8.0)]
        #[case(3.0, None, 3.0)]
        fn test_maybe_add(#[case] lhs: f32, #[case] rhs: Option<f32>, #[case] expected: f32) {
            assert_eq!(lhs.maybe_add(rhs), expected);
        }

        #[rstest]
        #[case(3.0, Some(5.0), -2.0)]
        #[case(5.0, Some(3.0), 2.0)]
        #[case(3.0, None, 3.0)]
        fn test_maybe_sub(#[case] lhs: f32, #[case] rhs: Option<f32>, #[case] expected: f32) {
            assert_eq!(lhs.maybe_sub(rhs), expected);
        }
    }
}
