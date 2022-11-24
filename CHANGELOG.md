# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `Extend` trait for `RBTree`
- `FusedIterator` trait for iterators over `RBTree` and `RBForest`
- Invariant checkers in tests for better error-catching
- Internal consistency checks, can be turned on via `test` or `fuzzing` features
- Fuzzing harnesses for `RBTree` and `RBForest`

### Changed
- Removed internal use of `unsafe` keyword, as it actually can not cause neither soundness problems nor memory safety bugs.
`from_slice()` methods are still marked as unsafe because the caller must ensure, that the slice was properly initialized
- Split`Debug` implementation into two variants: `cfg(test)` with all internal information about tree structure and `cfg(not(test))` - formatting just as in ordinary map

### Fixed
- A bug in one insertion case

## [0.1.0-alpha.1] - 2022-10-19
### Fixed
- Removed hidden dependency on `std` in `borsh`
## [0.1.0-alpha] - 2022-10-03
Initial release

[Unreleased]: https://github.com/solcery/slice-rbtree/compare/dev...HEAD
[0.1.0-alpha.1]: https://github.com/solcery/slice-rbtree/compare/v0.1.0-alpha...v0.1.0-alpha.1
[0.1.0-alpha]: https://github.com/solcery/slice-rbtree/releases/tag/v0.1.0-alpha
