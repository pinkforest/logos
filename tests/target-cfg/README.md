# Target/cfg specific integration tests

These test that `#[no_std]` + no-alloc compiles correctly.

Together with all the various cfg & feature combinations.

Any cfg / target-specific tests should be added here.

In the long run trybuild may gain support to replace this:
- https://github.com/dtolnay/trybuild/issues/326
