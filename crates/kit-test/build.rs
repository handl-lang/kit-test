//! Vendors the `lib-rand-stub` package into `OUT_DIR` (ADR-0160 P3): the package
//! repo ships no `Cargo.toml`, and this crate must stay hermetic.
fn main() {
    handl_pkg_vendor::vendor("../../../lib-rand-stub").expect("vendor lib-rand-stub");
    handl_pkg_vendor::vendor("../../../lib-test").expect("vendor lib-test");
}
