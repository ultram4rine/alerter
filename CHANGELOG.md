# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Build Docker image for ARMv7.

### Changed

- Use rust:1.61 as base image for docker image.
- Use libssl-dev:armhf for build deb package for ARMv7.

## [0.3.1] - 2022-05-02

### Added

- Build RPM package for CentOS 7 too.
- Added changelog to Deb packages.
- Added man page to Deb/RPM packages.

### Changed

- Remove apt cache from docker image.
- Use rust:1.60 as base image for docker image.

### Fixed

- Fixed license field in RPM packages
- Fixed home directory in post installation script.
- Fixed extended description in Deb packages.

## [0.3.0] - 2022-03-15

### Added

- [Matrix](https://matrix.org/) support.

### Changed

- Change default listen port to `3030`.

### Fixed

- Fix Docker image.

## [0.2.3] - 2022-03-14

### Changed

- New Docker image.

### Fixed

- Fix logging.

## [0.2.2] - 2022-03-11

### Added

- Use [clap](https://github.com/clap-rs/clap).

## [0.2.1] - 2022-02-21

### Changed

- Updated to Rust 2021 Edition.

## [0.2.0] - 2021-08-30

### Changed

- Use [teloxide](https://github.com/teloxide/teloxide).

## [0.1.0] - 2021-08-30

### Changed

- Rewrited in [Rust](https://www.rust-lang.org/).

## [0.0.1] - 2021-06-16

- Workable version.
