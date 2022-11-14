# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Implemented `Extend` trait for `RBTree`
### Changed
- Removed internal use of `unsafe` keyword, as it actually can not cause neither soundness problems nor memory safety bugs.
`from_slice()` methods are still marked as unsafe because the caller must ensure, that the slice was properly initialized

# [0.1.0-alpha.1] - 2022-10-19
## Fixed
- Removed hidden dependency on `std` in `borsh`
# [0.1.0-alpha] - 2022-10-03
Initial release

[Unreleased]: https://github.com/solcery/slice-rbtree/compare/dev...HEAD
[0.1.0-alpha.1]: https://github.com/solcery/slice-rbtree/compare/v0.1.0-alpha...v0.1.0-alpha.1
[0.1.0-alpha]: https://github.com/solcery/slice-rbtree/releases/tag/v0.1.0-alpha
