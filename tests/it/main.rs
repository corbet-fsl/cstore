//! All integration tests in one binary: each former `tests/*.rs` file is a
//! module here, so the crate's dependency closure is linked once per test run
//! instead of once per file. Add new integration tests as modules below; keep a
//! file separate only for process-global state, `harness = false`, required
//! features, or a CI tier that must run it alone.

mod filesystem;
mod model;
