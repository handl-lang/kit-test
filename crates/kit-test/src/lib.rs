//! `handl-kit-test` — the test-framework kit.
//!
//! Extracted from `handl-kit-canonical` (ADR-0092): the pure-Handl
//! `test`/`property`/`benchmark`/`fuzz`/`snapshot`/`assert` modules plus
//! `rand-stub`, the deterministic no-effect `rand` backend the framework
//! needs. A host that runs handl tests links this kit and calls
//! [`register`] before building its module registry.

use std::sync::Arc;

use handl_kit_core::{KitResolver, ModuleEntry, ModuleCtor, build_type_module};
use handl_plugin_lib_handl::HandlModule;
use handl_plugin_lib_stdlib as stdlib;
use handl_plugin_lib_tag::TagModule;

// ---------------------------------------------------------------------------
// test framework modules (pure Handl — ADR-0092)
// ---------------------------------------------------------------------------

const TEST_HNL: &str = include_str!("../../../../lib-test/src/test.hnl");
const TEST_FUNCS: &[&str] = &["run"];

const PROPERTY_HNL: &str = include_str!("../../../../lib-test/src/property.hnl");
const PROPERTY_FUNCS: &[&str] = &["for_all"];

const BENCHMARK_HNL: &str = include_str!("../../../../lib-test/src/benchmark.hnl");
const BENCHMARK_FUNCS: &[&str] = &["run"];

const FUZZ_HNL: &str = include_str!("../../../../lib-test/src/fuzz.hnl");
const FUZZ_FUNCS: &[&str] = &["run"];

// rand-stub — the deterministic, no-effect `rand` backend used by the
// test framework (`test`/`property`/`fuzz` modules need `rand.*`).
const RAND_STUB_HNL: &str = include_str!("../../../../lib-rand-stub/src/index.hnl");
const RAND_STUB_FUNCS: &[&str] = &[
    "int",
    "float",
    "bool",
    "pick",
    "weighted_pick",
    "choice",
    "sample",
    "shuffle",
    "bytes",
    "string",
    "gauss",
    "exponential",
    "poisson",
    "binomial",
    "seeded_int",
    "seeded_float",
    "seeded_bool",
    "seeded_pick",
    "seeded_choice",
    "seeded_sample",
    "seeded_shuffle",
    "seeded_string",
    "seeded_gauss",
];

const SNAPSHOT_HNL: &str = include_str!("../../../../lib-test/src/snapshot.hnl");
const SNAPSHOT_FUNCS: &[&str] = &["format", "match"];

const ASSERT_HNL: &str = include_str!("../../../../lib-test/src/assert.hnl");
const ASSERT_FUNCS: &[&str] = &["equal", "not_equal", "true", "false", "contains"];

// ---------------------------------------------------------------------------
// test framework module builders (ADR-0092)
// ---------------------------------------------------------------------------

fn build_test_module() -> HandlModule {
    HandlModule::new("test", TEST_HNL, TEST_FUNCS)
        .with_dependency_arc(std::sync::Arc::new(TagModule))
        .with_dependency_arc(std::sync::Arc::new(build_rand_stub_module()))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module_with("serialize", Arc::new(KitResolver)).expect("serialize"),
        ))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module("combinators").expect("combinators"),
        ))
        .with_dependency_arc(std::sync::Arc::new(build_type_module()))
}

fn build_property_module() -> HandlModule {
    HandlModule::new("property", PROPERTY_HNL, PROPERTY_FUNCS)
        .with_dependency_arc(std::sync::Arc::new(TagModule))
        .with_dependency_arc(std::sync::Arc::new(build_rand_stub_module()))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module_with("serialize", Arc::new(KitResolver)).expect("serialize"),
        ))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module("combinators").expect("combinators"),
        ))
}

fn build_benchmark_module() -> HandlModule {
    HandlModule::new("benchmark", BENCHMARK_HNL, BENCHMARK_FUNCS)
        .with_dependency_arc(std::sync::Arc::new(TagModule))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module_with("serialize", Arc::new(KitResolver)).expect("serialize"),
        ))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module("combinators").expect("combinators"),
        ))
}

fn build_fuzz_module() -> HandlModule {
    HandlModule::new("fuzz", FUZZ_HNL, FUZZ_FUNCS)
        .with_dependency_arc(std::sync::Arc::new(TagModule))
        .with_dependency_arc(std::sync::Arc::new(build_rand_stub_module()))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module_with("serialize", Arc::new(KitResolver)).expect("serialize"),
        ))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module("combinators").expect("combinators"),
        ))
}

fn build_snapshot_module() -> HandlModule {
    HandlModule::new("snapshot", SNAPSHOT_HNL, SNAPSHOT_FUNCS)
        .with_dependency_arc(std::sync::Arc::new(TagModule))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module_with("serialize", Arc::new(KitResolver)).expect("serialize"),
        ))
}

fn build_assert_module() -> HandlModule {
    HandlModule::new("assert", ASSERT_HNL, ASSERT_FUNCS)
        .with_dependency_arc(std::sync::Arc::new(TagModule))
        .with_dependency_arc(std::sync::Arc::new(
            stdlib::module_with("serialize", Arc::new(KitResolver)).expect("serialize"),
        ))
}

/// Build the deterministic, pure-Handl `rand` backend from `rand-stub`.
fn build_rand_stub_module() -> HandlModule {
    HandlModule::new("rand", RAND_STUB_HNL, RAND_STUB_FUNCS).with_global_lookup(
        std::sync::Arc::new(stdlib::TableModuleLookup::new(std::sync::Arc::new(
            KitResolver,
        ))),
    )
}

// ---------------------------------------------------------------------------
// registration
// ---------------------------------------------------------------------------

/// The kit's module table — `test`/`property`/`benchmark`/`fuzz`/
/// `snapshot`/`assert` plus the `rand` stub backend they depend on.
pub const MODULES: &[(&'static str, ModuleCtor)] = &[
    ("test", |_, e| ModuleEntry::Handl(build_test_module().with_eval_engine(e))),
    ("property", |_, e| {
        ModuleEntry::Handl(build_property_module().with_eval_engine(e))
    }),
    ("benchmark", |_, e| {
        ModuleEntry::Handl(build_benchmark_module().with_eval_engine(e))
    }),
    ("fuzz", |_, e| ModuleEntry::Handl(build_fuzz_module().with_eval_engine(e))),
    ("snapshot", |_, e| {
        ModuleEntry::Handl(build_snapshot_module().with_eval_engine(e))
    }),
    ("assert", |_, e| {
        ModuleEntry::Handl(build_assert_module().with_eval_engine(e))
    }),
    ("rand", |_, e| {
        ModuleEntry::Handl(build_rand_stub_module().with_eval_engine(e))
    }),
];

/// Register the test framework's modules into the shared kit table.
/// `Once`-safe to call repeatedly.
pub fn register() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| handl_kit_core::register_modules(MODULES));
}
