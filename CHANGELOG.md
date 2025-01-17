# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.1] - 2021-03-14
### Changed
- Use local time instead of UTC for search (Issue [#77](https://github.com/out-of-cheese-error/gooseberry/issues/77))
- Updated dependencies

### Fixed
- markdown preview in search (Issue [#74](https://github.com/out-of-cheese-error/gooseberry/issues/74))

## [0.8.0] - 2021-03-14
### Fixed
* All w3 selectors (partially) supported (Issue [#66](https://github.com/out-of-cheese-error/gooseberry/issues/66))

## [0.7.1] - 2021-03-29

### Added

Raw annotations to page template

## [0.7.0] - 2021-03-26

### Fixed

* truncates filename if over 250 characters

### Added

* The web-page/document `title` can be used in the annotation template, hierarchy, and sort configurations (
  Issue [#69](https://github.com/out-of-cheese-error/gooseberry/issues/69))
* `gooseberry uri` and `Shift-Up` option to `gooseberry search` that prints out the set of URIs associated with a list of selected annotations.

### Changed

* Updated dependencies

## [0.6.0] - 2021-03-10

### Added

* Tag manager that displays a search window of existing tags to add/remove and allows creating new tags (
  Issue [#63](https://github.com/out-of-cheese-error/gooseberry/issues/63))
* `ignore_tags` config option (Issue [#60](https://github.com/out-of-cheese-error/gooseberry/issues/60))
* Add/remove multiple tags at once using comma-separated input e.g `gooseberry tag --from=today tag1,tag2,tag3`

### Changed

* Updated dependencies

## [0.5.2] - 2020-01-21

### Fixed

* Use DEFAULT_PAGE_TEMPLATE in editor
* Remove trailing "/" from URLs before converting them into filenames (Issue [#57](https://github.com/out-of-cheese-error/gooseberry/issues/57))

### Added

* Use --config or -c to open gooseberry with a specific config file. If empty, takes from GOOSEBERRY_CONFIG environment variable or uses default
  location (Issue [#54](https://github.com/out-of-cheese-error/gooseberry/issues/54))

## [0.5.1] - 2020-01-21

### Fixed

* URI file name should have the full path

## [0.5.0] - 2020-01-20

### Fixed

* Fixed Issue [#49](https://github.com/out-of-cheese-error/gooseberry/issues/49) - recursively creates db_dir and kb_dir
* Fixed `search` and `view` without annotation_template set

### Added

* Sort option `gooseberry config kb sort` (Issue [#48](https://github.com/out-of-cheese-error/gooseberry/issues/48))
* Page template option `gooseberry config kb page` (Issue [#52](https://github.com/out-of-cheese-error/gooseberry/issues/52))

### Changed

* URI/BaseURI options don't have "http" and "https" in folder/file name anymore (these are also not used when sorting)

## [0.4.0] - 2021-01-19

### Changed

* escape spaces in links with "%20". See https://github.com/rust-lang/mdBook/issues/527
* remove IGNORE_TAG business, delete always deletes from hypothesis
* list available groups on running `gooseberry config group` with the "use existing" option

## [0.3.0] - 2021-01-17

### Changed:

* tokio 1.0 update
* Fixed some bugs
* Added a make test

## [0.2.0-alpha] - 2021-01-16

Switching to Handlebars templates instead of restricting to mdBook-style wiki (Major change). Tests don't work right now.

## [0.1.1] - 2020-11-28

### Changed:

* hypothesis crate points to crates.io version instead of git
* upgraded dependencies (except tokio and directories-next)

### Added:

* badges to README
* link to releases in README
* first crates.io version

## [0.1.0] - 2020-11-28
First somewhat decent release!

Main commands:

* `gooseberry sync` - syncs hypothesis annotations to gooseberry
* `gooseberry search` - opens an interactive search buffer to select annotations. Has keyboard shortcuts to add tags, remove tags and delete
  annotations. This should be the main entrypoint for users while `gooseberry tag`, `gooseberry delete`, and `gooseberry view`
  are more for automating these tasks.
* `gooseberry make` - builds the mdbook knowledge base
* `gooseberry config` - manages configuration, view and edit Hypothesis credentials, the Hypothesis group, and the location of the knowledge base
* `gooseberry move` - move annotations from one group to another (**move** not copy). Useful if you have a bunch of annotations scattered around and
  want to move them into one group for gooseberry.

[0.8.1]: https://github.com/out-of-cheese-error/gooseberry/compare/0.8.0...0.8.1

[0.8.0]: https://github.com/out-of-cheese-error/gooseberry/compare/0.7.1...0.8.0

[0.7.1]: https://github.com/out-of-cheese-error/gooseberry/compare/0.7.0...0.7.1

[0.7.0]: https://github.com/out-of-cheese-error/gooseberry/compare/0.6.0...0.7.0

[0.6.0]: https://github.com/out-of-cheese-error/gooseberry/compare/0.5.2...0.6.0

[0.5.2]: https://github.com/out-of-cheese-error/gooseberry/compare/0.5.1...0.5.2

[0.5.1]: https://github.com/out-of-cheese-error/gooseberry/compare/0.5.0...0.5.1

[0.5.0]: https://github.com/out-of-cheese-error/gooseberry/compare/0.4.0...0.5.0

[0.4.0]: https://github.com/out-of-cheese-error/gooseberry/compare/0.3.0...0.4.0

[0.3.0]: https://github.com/out-of-cheese-error/gooseberry/compare/0.2.0-alpha...0.3.0

[0.2.0-alpha]: https://github.com/out-of-cheese-error/gooseberry/compare/0.1.1...0.2.0-alpha

[0.1.1]: https://github.com/out-of-cheese-error/gooseberry/compare/0.1.0...0.1.1

[0.1.0]: https://github.com/out-of-cheese-error/gooseberry/releases/tag/0.1.0
