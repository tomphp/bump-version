# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.4.4](https://github.com/tomphp/versioned-files/compare/v0.4.3..v0.4.4) - 2023-10-26
#### Bug Fixes
- Remove duplicate output - ([28fe717](https://github.com/tomphp/versioned-files/commit/28fe717432bb35d9e4b1ce10a9e17473ada268ea)) - Tom Oram
- Add a formatter - ([3dfc5fd](https://github.com/tomphp/versioned-files/commit/3dfc5fdda68d0ee14fe2edb0327bab3c79a3f48f)) - Billie Thompson
#### Refactoring
- Use UUIDs to tie events together - ([bf4fc93](https://github.com/tomphp/versioned-files/commit/bf4fc93de4b79cb3eeb686f9cc0e08ebfa80ac20)) - Tom Oram
- Move update logic to commands::update - ([9fb1b82](https://github.com/tomphp/versioned-files/commit/9fb1b828e2bebabba4561c08494c0d3baab880b1)) - Tom Oram
- Extract Command - ([5ac9e7b](https://github.com/tomphp/versioned-files/commit/5ac9e7b910387fc7e559b0f35f6855c5552cf14b)) - Tom Oram
- Extract Formatter - ([c8c9138](https://github.com/tomphp/versioned-files/commit/c8c9138c49c91abbc767b580df10d6c3ac7dd5fc)) - Tom Oram
- Move app state into own module - ([b0cecfc](https://github.com/tomphp/versioned-files/commit/b0cecfc4c5ab6d373ee87f21b4415424b55129c7)) - Billie Thompson
- Extract app state into struct - ([22325fd](https://github.com/tomphp/versioned-files/commit/22325fdf102604aec08239d8bbbb08903b75f13b)) - Billie Thompson

- - -

## [v0.4.3](https://github.com/tomphp/versioned-files/compare/v0.4.2..v0.4.3) - 2023-10-26
#### Bug Fixes
- Lift the process exit to main - ([5f5ac06](https://github.com/tomphp/versioned-files/commit/5f5ac063652a4d9c1364bffd49a6467163413fe0)) - Billie Thompson
#### Continuous Integration
- Use make in the pipeline - ([4b18179](https://github.com/tomphp/versioned-files/commit/4b1817902a6abbc05fa659664823ab94841ea0c1)) - Tom Oram
- Use pinned version of versioned-files action - ([0ae7cb4](https://github.com/tomphp/versioned-files/commit/0ae7cb46d8431d015c5acaf4d4d43e6323cd472c)) - Tom Oram
- Restructure job dependencies - ([9d26f1d](https://github.com/tomphp/versioned-files/commit/9d26f1dbe890d69a507273ed99efa4b702a4712c)) - Tom Oram
#### Miscellaneous Chores
- Add versioned-files.exe to .gitignore - ([d1366c0](https://github.com/tomphp/versioned-files/commit/d1366c065f24475adebd60957853ee12d995cba8)) - Tom Oram
#### Refactoring
- **(update-command)** Use events to display output - ([20e46bb](https://github.com/tomphp/versioned-files/commit/20e46bb6b713cb2938311907a3ee1bef9a6b96e9)) - Tom Oram
- Formatting - ([7b21b86](https://github.com/tomphp/versioned-files/commit/7b21b8649c6bf76a3ffbf6503fecb22713762fa2)) - Billie Thompson
- Use futures::streams - ([e9e7f18](https://github.com/tomphp/versioned-files/commit/e9e7f185528b6660e655acac17f1444e4dad2e04)) - Tom Oram

- - -

## [v0.4.2](https://github.com/tomphp/versioned-files/compare/v0.4.1..v0.4.2) - 2023-10-24
#### Bug Fixes
- **(ci)** Use the versioned-files action to bump versions - ([40ed363](https://github.com/tomphp/versioned-files/commit/40ed36339a008d272026e9ceb0701e79d8db225c)) - Tom Oram
- **(versioned-files-action)** Use lastest version - ([0b54281](https://github.com/tomphp/versioned-files/commit/0b54281f957ccd062fb3d49e9ac11b862c2ba3bc)) - Tom Oram
#### Continuous Integration
- Temporarily use use versioned-files action directly - ([1f2e056](https://github.com/tomphp/versioned-files/commit/1f2e0566614ee69058a905506bf9201a97cf8d17)) - Tom Oram

- - -

## [v0.4.1](https://github.com/tomphp/versioned-files/compare/v0.4.0..v0.4.1) - 2023-10-24
#### Bug Fixes
- **(ci)** Use the versioned-files action to bump versions - ([d8cca23](https://github.com/tomphp/versioned-files/commit/d8cca236178b66dd3ff18fed1dd1b6f5d5d054d0)) - Tom Oram
- **(update-command)** Wait for cargo-edit to install - ([06f72d7](https://github.com/tomphp/versioned-files/commit/06f72d7572fb23bd2897aa72e2803f84f2fb3dc4)) - Tom Oram
#### Build system
- Create make target to build release version - ([b0f9b62](https://github.com/tomphp/versioned-files/commit/b0f9b629c94d26f409780b29f0d7f1a9d1110973)) - Tom Oram
#### Continuous Integration
- Temporarily use built version to bump - ([a6a979e](https://github.com/tomphp/versioned-files/commit/a6a979e5e9e2b44fd72c310e79b43d9efcc74289)) - Tom Oram
#### Style
- Fix lint issues - ([ed5eff6](https://github.com/tomphp/versioned-files/commit/ed5eff6e272f4f1d1f7670ab201411cce79f514f)) - Tom Oram

- - -

## [v0.4.0](https://github.com/tomphp/versioned-files/compare/v0.3.0..v0.4.0) - 2023-10-24
#### Features
- **(versioned-files-action)** Create action - ([5b6638b](https://github.com/tomphp/versioned-files/commit/5b6638bb69870c53d9066bd7e1bc41634fcff378)) - Tom Oram

- - -

## [v0.3.0](https://github.com/tomphp/versioned-files/compare/v0.2.2..v0.3.0) - 2023-10-24
#### Documentation
- Update version - ([2559870](https://github.com/tomphp/versioned-files/commit/25598708681546a6e1d83cb7a688e7f979c69d60)) - Tom Oram
#### Features
- **(update-command)** Add !cargo - ([ec16aaa](https://github.com/tomphp/versioned-files/commit/ec16aaa020af4f465242635521a69c5e9b182866)) - Tom Oram
#### Style
- Apply code styles - ([8743505](https://github.com/tomphp/versioned-files/commit/8743505c6f6bac153617cd7bf695a8854ee94548)) - Tom Oram

- - -

## [v0.2.2](https://github.com/tomphp/versioned-files/compare/v0.2.1..v0.2.2) - 2023-10-24
#### Bug Fixes
- **(build)** Compile binary with correct version number - ([e4cfc2d](https://github.com/tomphp/versioned-files/commit/e4cfc2d615aa957cc55cb56097ee67560774b589)) - Tom Oram

- - -

## [v0.2.1](https://github.com/tomphp/versioned-files/compare/v0.2.0..v0.2.1) - 2023-10-24
#### Bug Fixes
- **(ci)** Update the version number - ([a767887](https://github.com/tomphp/versioned-files/commit/a767887cd301eecbc6962d8f65128aa0c7c25cba)) - Tom Oram
#### Continuous Integration
- Use tomphp/github-actions/checkout-rust-project - ([dfc7d36](https://github.com/tomphp/versioned-files/commit/dfc7d368234f42f00665552820a436bccc415fe7)) - Tom Oram
- Require GitHub action tests to pass before releasing - ([025643b](https://github.com/tomphp/versioned-files/commit/025643b23393df5bc5a48e561eef421f8bc4df36)) - Tom Oram
#### Refactoring
- **(setup-versioned-files-action)** Use tomphp/github-actions/install-release-binary - ([fbcde17](https://github.com/tomphp/versioned-files/commit/fbcde175dad5eaede4fe3a59afd4523f6fa52f9d)) - Tom Oram
- **(setup-versioned-files-action)** Add Determine Binary Name step - ([1e933de](https://github.com/tomphp/versioned-files/commit/1e933de77f8a945e5b6040fb638b0fed67d595cc)) - Tom Oram

- - -

## [v0.2.0](https://github.com/tomphp/versioned-files/compare/v0.1.0..v0.2.0) - 2023-10-23
#### Features
- **(setup-versioned-files-action)** Create github action - ([cf92e86](https://github.com/tomphp/versioned-files/commit/cf92e864f3ae743cd5cec39e934b803d247de7ab)) - Tom Oram

- - -

## [v0.1.0](https://github.com/tomphp/versioned-files/compare/451c32208e96ae9521161741186f87c8546b2c69..v0.1.0) - 2023-10-23
#### Build system
- Add specdown - ([f576a20](https://github.com/tomphp/versioned-files/commit/f576a20b1dbcfb8a1b147ac97d12d7fa7c32c890)) - Tom Oram
- Initialize cargo project - ([e882c95](https://github.com/tomphp/versioned-files/commit/e882c9552b7ececcd99aeecc49b0c44416719869)) - Tom Oram
#### Continuous Integration
- **(dependabot)** Add dependabot and mergify config - ([083d833](https://github.com/tomphp/versioned-files/commit/083d833ce1952026a358e0e2bf153322898f1a04)) - Tom Oram
- Add pipeline - ([4b5e023](https://github.com/tomphp/versioned-files/commit/4b5e023af1435cb4a98ff4045da8b3a6a5bb4fb0)) - Tom Oram
#### Documentation
- Add current status warning - ([8108d37](https://github.com/tomphp/versioned-files/commit/8108d376c234fa56b66d54dc08abeeef80dd9c7c)) - Tom Oram
#### Features
- **(bump-cmd)** Implement string-pattern update - ([c6b4b58](https://github.com/tomphp/versioned-files/commit/c6b4b582dab13ad67afef3f8221f97ae2f5aec95)) - Tom Oram
- **(bump-cmd)** Error when bump-version.yml is missing - ([a75caeb](https://github.com/tomphp/versioned-files/commit/a75caeb6d8aa21297a5a66a6bb33f98db3bc7392)) - Tom Oram
- **(help)** Add documentation to command help - ([e97cec2](https://github.com/tomphp/versioned-files/commit/e97cec25042d9ac41fb54b85a1a7aa61a2630189)) - Tom Oram
#### Miscellaneous Chores
- **(bump-cmd)** Add logic to substitute string patterns - ([6a29630](https://github.com/tomphp/versioned-files/commit/6a29630852a278bcf478a2a24f91628387c39f9d)) - Tom Oram
- Rename to bump command to update - ([dd4f669](https://github.com/tomphp/versioned-files/commit/dd4f66947e4ad9028a77f6660c3639be35e00458)) - Tom Oram
- Rename to versioned-files - ([b4a97d7](https://github.com/tomphp/versioned-files/commit/b4a97d71f58c2138b5100ea5d5d9a5b86f80fdd0)) - Tom Oram
- Bootstrap the clap framework - ([99f86f7](https://github.com/tomphp/versioned-files/commit/99f86f7cf584c078333ccce8e12080b8b524a1a3)) - Tom Oram
#### Refactoring
- Move Config to config::Config - ([fe41245](https://github.com/tomphp/versioned-files/commit/fe41245f502b75dd37cf4162d347d804aa04f970)) - Tom Oram
- Move StringPatternConfig to string_pattern::Config - ([d94a9f8](https://github.com/tomphp/versioned-files/commit/d94a9f8f19068ef633a4e564ec0ea6ffb72d15ad)) - Tom Oram

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).