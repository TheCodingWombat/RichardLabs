# Composable Grant Proposal: Maintenance of Substrace and feature development

## Overview
Substrace is an open-source linting tool for Substrate development, implemented as part of one's continuous integration pipeline (CI), helping developers stay away from commonly error-prone patterns and reducing work load at the same time.

Further development to Substrace will extend its set of features to encompass more lints and make maintenance partially automized in the future. Maintenance of Substrace will be necessary as to keep it up to date with the latest Rust compiler version, since it does not yet have a stable API and calls may suddenly change.

## Project Details
Substrace will be published as an open source cargo package, compatible with the latest compiler internals. 

Development will at first make use of the `dylint` tool, but will soon switch to be stand-alone. Substrace can be run as a Github action so as to reduce CI run times.

Within this infrastructure the following lint will be update:
- `no-panics`: Checks that the script will never panic. A basic version is already available, which will be made independent of `clippy`.

And these four lints will be created:
- `missing-transactional`: Checks that non-atomic functions that perform multiple actions which can fail will be marked with the `#[transactional]` marker, so as to automatically revert the entire transaction if a part fails.
- `enable-singlepass-benchmark`:  Ensures that the necessary markers are present so that benchmarks are (only) compiled and run during unit tests.
- `xcm-config-check`:  A lint that validates whether `pallet-xcm` has been configure properly, trying to prevent errors such as https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb.
- `extrinsics-reordered`: This lint detects when extrinsics are reordered, which can cause errors if these reorderings are not propagated correctly everywhere, causing data to be interpreted wrongly.

If time allows an additional library `path-gen` will be supplied which generates paths for use within linters, ensuring that Substraces does not fail silently when `substrate` (trivially) changes its crate layout

In the end Substrace will be a useful linting tool with a very basic set of lints that shows its potential and which can later be extended. It will be easily addable to one's CI, easing development within the Web3 ecosystem all around.

## Milestones
To reach a version of Substrace as envisioned in the section above, a set of milestones (completely 1-1 to features above) has been compiled.

### Infrastructure related.

- [ ] Host the Substrace repository and publish as a cargo package.
- [ ] Update Substrace to be compatible with the latest compiler internals.
- [ ] Provide Substrace as stand-alone tool independent of `dylint`.
- [ ] Create Github action for running substrace (for better CI times).

The funding need to complete the above milestones is $2000, evenly spread over each milestone.

### Product related.

- [ ] Update `no-panics` lint to perform checks by itself, removing the dependency on `clippy`.
- [ ] Add the `missing-transactional` lint for detecting extrinsics without the `#[transactional]` marker.
- [ ] Add the `enable-singlepass-benchmarks` lint to verify that benchmarks are compiled and run during unit tests (single pass).
- [ ] Add the `xcm-config-check` lint to validate if `pallet-xcm` has been configured properly.
- [ ] Add the `extrinsics-reordered` lint and subcommand to detect extrinsic reorderings.

The funding need to complete the above milestones is $9500, evenly spread over each milestone.

### Optional

- [ ] Create the `path_gen` library.

The funding need to complete the above milestone is $1000.

## Total Funding Need
To complete this grant in its entirety, $12500 of funding is required.

## Ecosystem Fit
Substrace will not only be useful for development within Composable, but will also be useful for other ventures using Substrate and beyond since the lints in this initial grant proposal are independent of their use within Composable. In this way, Substrace will help the Web3 ecosystem be more resilient to human error and malicious intent, as well as helping developers more easily check a pull request or commit for common errors that are now contained in the lints. This last advantage will make it easier for companies to hire new developers that do not yet know about the intrinsics of what these lints try to prevent.

Future work encompasses presenting the features implemented in this grant proposal at conferences to garner more funding for guaranteeing continued maintenance and extending the feature set of Substrace. Apart from CI integration, extensions for IDEs and text editors may also be desirable. Automatically crawling Substrate repositories on Github and running Substrace on it can be employed to show more people the power of Substrace.