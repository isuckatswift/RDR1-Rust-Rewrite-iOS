# Red Dead Redemption - iOS Rust Port

An experimental native iOS port of Red Dead Redemption (2010), focused on performance, compatibility, and Apple silicon optimisation.


## Status: Planning and research.

This repository is not currently a playable port. It does not contain the original game’s source code or assets.

## Project goals

* Investigate the feasibility of an iOS implementation.
* Explore Rust for suitable game systems and supporting tools.
* Investigate native Apple graphics technologies, including Metal.
* Develop a modular architecture.
* Measure performance on Apple Silicon devices.
* Document development progress and technical findings.

## Technology

Technology	Intended purpose
> Rust	Portable systems and tooling
> C++	Potential engine components
> Objective-C++	Native iOS integration
> Metal	Graphics rendering
> Xcode	iOS compilation and signing

These technologies are candidates, not evidence of an existing implementation.

## Proposed structure

RDR1-Rust-Rewrite-iOS/
├── README.md
├── docs/
├── crates/
├── ios/
├── native/
└── tests/

## Development roadmap

1. Research the original game’s technical requirements.
2. Establish a realistic architecture.
3. Create a minimal iOS application.
4. Prototype individual systems.
5. Investigate rendering, input, and audio integration.
6. Profile performance and optimise where possible.

## Important notes

A complete port requires access to suitable source code, compatible engine components, and the necessary legal permissions.

The project will not distribute proprietary game files or copyrighted assets.

## Disclaimer

Red Dead Redemption is the property of its respective rights holders. This is an independent, unofficial research project and is not affiliated with or endorsed by its publisher or developers.
