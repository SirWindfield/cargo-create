# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [HEAD](https://github.com/SirWindfield/cargo-create/compare/v0.3.1...HEAD) (2020-08-24)

## [v0.3.1](https://github.com/SirWindfield/cargo-create/compare/v0.3.0...v0.3.1) (2020-08-24)

### Features

- move/rename files ([50eb420](https://github.com/SirWindfield/cargo-create/commit/50eb42040d767f667fef2047e8a9691baee2fe67)) , closes [#32](https://github.com/SirWindfield/cargo-create/issues/32)
- add new text transformation Tera filters ([e2bc82b](https://github.com/SirWindfield/cargo-create/commit/e2bc82b6541ad13fa900bcd8fb93edaeb7f58668)) , closes [#44](https://github.com/SirWindfield/cargo-create/issues/44)

# [v0.3.0](https://github.com/SirWindfield/cargo-create/compare/v0.2.2...v0.3.0) (2020-08-07)

### Bug Fixes

- `-b` usage together with `-p` ([f976b7e](https://github.com/SirWindfield/cargo-create/commit/f976b7e46ece12e9c31eba9e4c855937bbdde313))
- template features not correctly resolvable ([aa41f5b](https://github.com/SirWindfield/cargo-create/commit/aa41f5b44fd8ce3b8106164407c68414565dbfde)) , closes [#34](https://github.com/SirWindfield/cargo-create/issues/34)

## [v0.2.2](https://github.com/SirWindfield/cargo-create/compare/v0.2.1...v0.2.2) (2020-08-05)

### Bug Fixes

- super features not copying files ([c04a212](https://github.com/SirWindfield/cargo-create/commit/c04a2129ddbbf72e7cc43202182e3ef34ec0a5ea)) , closes [#22](https://github.com/SirWindfield/cargo-create/issues/22)

## [v0.2.1](https://github.com/SirWindfield/cargo-create/compare/v0.2.0...v0.2.1) (2020-08-05)

### Bug Fixes

- missing `DefaultVariableProvider` ([d0430d3](https://github.com/SirWindfield/cargo-create/commit/d0430d33a73b685d2e6686486242e54ea3bfd3a8)) , closes [#6](https://github.com/SirWindfield/cargo-create/issues/6)

# [v0.2.0](https://github.com/SirWindfield/cargo-create/compare/v0.1.6...v0.2.0) (2020-08-05)

### Bug Fixes

- config flag conflicts not set correctly ([5aea258](https://github.com/SirWindfield/cargo-create/commit/5aea2581000989a9d430144c51c1e14824b4c0c3)) , closes [#26](https://github.com/SirWindfield/cargo-create/issues/26)

### Features

- force flag ([bbb0b5c](https://github.com/SirWindfield/cargo-create/commit/bbb0b5c1934356e81483ba028aafc50f04f178cf)) , closes [#23](https://github.com/SirWindfield/cargo-create/issues/23)

## [v0.1.6](https://github.com/SirWindfield/cargo-create/compare/v0.1.5...v0.1.6) (2020-08-05)

### Features

- colored help output ([0988bfa](https://github.com/SirWindfield/cargo-create/commit/0988bfabb9c25df38a62343149fc0d7a7edfde6d)) , closes [#24](https://github.com/SirWindfield/cargo-create/issues/24)

## [v0.1.5](https://github.com/SirWindfield/cargo-create/compare/v0.1.4...v0.1.5) (2020-08-05)

### Features

- add branch option ([fa4a9ba](https://github.com/SirWindfield/cargo-create/commit/fa4a9bab206047ba2d2fde7c0c49580445b8d3c0)) , closes [#16](https://github.com/SirWindfield/cargo-create/issues/16)

## [v0.1.4](https://github.com/SirWindfield/cargo-create/compare/v0.1.3...v0.1.4) (2020-08-05)

### Bug Fixes

- **cli**: cargo subcommand not working properly ([cab654c](https://github.com/SirWindfield/cargo-create/commit/cab654ced7fb31f944cf6483eea74db7775daaa6)) , closes [#13](https://github.com/SirWindfield/cargo-create/issues/13)
- `name` being required if `config-path` is set ([e936e6f](https://github.com/SirWindfield/cargo-create/commit/e936e6fe155dd531c078a4df65195ce8f8a85f90)) , closes [#11](https://github.com/SirWindfield/cargo-create/issues/11)

## [v0.1.3](https://github.com/SirWindfield/cargo-create/compare/v0.1.2...v0.1.3) (2020-08-03)

### Bug Fixes

- panic when user config not found ([e5b75cb](https://github.com/SirWindfield/cargo-create/commit/e5b75cb1b89272706f39bbb0c9a059ada6bce814)) , closes [#5](https://github.com/SirWindfield/cargo-create/issues/5)
- user config values being mandatory ([17cde96](https://github.com/SirWindfield/cargo-create/commit/17cde960da4a4ec91485f69dc1afcc9216ceb92e)) , closes [#8](https://github.com/SirWindfield/cargo-create/issues/8)

## [v0.1.2](https://github.com/SirWindfield/cargo-create/compare/v0.1.1...v0.1.2) (2020-08-03)

### Bug Fixes

- panic when repository has already been cloned ([7e4199e](https://github.com/SirWindfield/cargo-create/commit/7e4199edbbc31a161aec92791ec1b41e3c14150b)) , closes [#2](https://github.com/SirWindfield/cargo-create/issues/2)
- user config not being properly detected ([9b8d29a](https://github.com/SirWindfield/cargo-create/commit/9b8d29afa84103cf96fb0b3a793dbcaab22d8c16)) , closes [#9](https://github.com/SirWindfield/cargo-create/issues/9)

## [v0.1.1](https://github.com/SirWindfield/cargo-create/compare/v0.1.0...v0.1.1) (2020-08-03)

### Bug Fixes

- add `features` template variable ([6b40a64](https://github.com/SirWindfield/cargo-create/commit/6b40a6466577ac26413daca4d201adf9f0008063)) , closes [#1](https://github.com/SirWindfield/cargo-create/issues/1)
- add cleanup emoji ([ddf47ba](https://github.com/SirWindfield/cargo-create/commit/ddf47ba1851979e7fe0c54cb450dec66726c3cf4))

# [v0.1.0](https://github.com/SirWindfield/cargo-create/compare/v0.0.0...v0.1.0) (2020-08-03)

