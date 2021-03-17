# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.2.0] - 2021-03-16
# Changed

- (BREAKING) disallow multiple `doc` attributes in display impl
  [https://github.com/yaahc/displaydoc/pull/22]. Allowing and ignoring extra
  doc attributes made it too easy to accidentally create a broken display
  implementation with missing context without realizing it, this change turns
  that into a hard error and directs users towards block comments if multiple
  lines are needed.

<!-- next-url -->
[Unreleased]: https://github.com/yaahc/displaydoc/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/yaahc/displaydoc/releases/tag/v0.2.0
