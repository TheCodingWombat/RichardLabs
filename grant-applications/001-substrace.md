# Composable Grant Proposal: Maintenance of Substrace and feature development

## Overview
// context: 
//  - what is substrace,       
//  - why does it need further feature development
//  - why does it need maintenance

## Project Details
// context:
//  - what will we do exactly
//  - what state will substrace be in at the end.

## Milestones
// context:
//  - individual milestones + funding need

### Infrastructure related.

- [ ] Host the Substrace repository and publish as a cargo package.
- [ ] Update Substrace to be compatible with the latest compiler internals.
- [ ] Provide Substrace as stand-alone tool independent of `dylint`.
- [ ] Create github action for running substrace (for better CI times).

The funding need to complete the above milestones is $2000, evenly spread over each milestone.

### Product related.

- [ ] Update `no-panics` lint to perform checks by itself, removing the dependency on `clippy`.
- [ ] Add the `missing-transactional` lint for detecting extrinsics without the `#[transactional]` marker.
- [ ] Add the `enable-singlepass-benchmarks` lint to verify that benchmarks are compiled and run during unit tests (single pass).
- [ ] Add the `xcm-config-check` lint to validate if `pallet-xcm` has been configured properly, checking on errors such as https://medium.com/kusama-network/kusamas-governance-thwarts-would-be-attacker-9023180f6fb.
- [ ] Add the `extrinsics-reordered` lint and subcommand to detect extrinsic reorderings.

The funding need to complete the above milestones is $8500, evenly spread over each milestone.

### Optional

- [ ] Create `path_gen`, a library to generate paths for use within linters, ensuring that Substrace does not fail silently when `substrate` changes the crate layout.

The funding need to complete the above milestones is $1000, evenly spread over each milestone.

## Total Funding Need
To complete this grant in its entirety, $11500 of funding is required.

## Ecosystem Fit
// context:
//  - what will substrace provide to the web3 ecosystem
//  - how can we base future work off this grant? (conference presentation, etc)