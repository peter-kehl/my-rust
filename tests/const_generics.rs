/// The goal is to return const/static instances from a const generic-parameterized function.
/// Alternative view: A way to provide multiple implementations of the same const generics-parameterized function.
/// Thanks to Kevin Reid https://github.com/kpreid for this pattern.
trait Helper {
    fn bounds() -> Self;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BoundsResult<const IS_DECIMAL: bool> {}

const BOUNDS_BINARY: BoundsResult::<{ false }> = BoundsResult {};
const BOUNDS_DECIMAL: BoundsResult::<{ true }> = BoundsResult {};

impl Helper for BoundsResult<false> {
    fn bounds() -> Self { BOUNDS_BINARY }
}
impl Helper for BoundsResult<true> {
    fn bounds() -> Self { BOUNDS_DECIMAL }
}
struct Client {}
impl Client {
    pub fn bounds<const IS_DECIMAL: bool>() -> BoundsResult<IS_DECIMAL>
    where
        BoundsResult<IS_DECIMAL>: Helper
    {
        Helper::bounds()
    }
}
