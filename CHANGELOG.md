## 0.3.4 (2025-05-22)

### Fix

- **nix**: update dependencies

## 0.3.3 (2025-05-08)

### Fix

- **ci**: revert version bump nix integration
- **ci**: add missing rust toolchain
- **build**: correct commitizen bump versioning

## 0.3.2 (2025-05-08)

### Fix

- **ureq**: update request for breaking changes

## 0.3.1 (2024-10-02)

### Fix

- **parser**: add work-around for new `sentences`

## 0.3.0 (2023-10-28)

### Feat

- add `features` for `http` client selection
- **retriever**: add `ureq` as http-client

### Fix

- `isahc` import
- **retriever**: convert to `reqwest`
- **compare**: executable path

### Refactor

- rename base `retriever.rs` with `isahc`
- **compare**: move related files to their dir

## 0.2.0 (2023-04-23)

### Feat

- add `dialoguer` for suggestions
- implement a generic parser (#4)
- convert builder `Request` to `HttpClient`
- convert `chttp` to `isahc`
- **reqwest**: re-add `reqwest` back-end
- convert requests from `reqwest` to `chttp`
- **mod**: export to modules
- add cli word acceptor
- add a simple valid result retriever
- add a flag whether to write response to file
- add suggestion list to translation result
- add basic translation result
- add sample html parsing with selector
- save response html to file
- export `RusTurengError` to seperate file
- convert response to content fetching
- export to functions
- simplified request for troubleshooting
- convert to a request to original site
- add basic reqwest json with tokio
- initial commit

### Fix

- **clippy**: redundant references
- **display**: naive right align indices
- **display**: add space between different tables
- uppercase query error
- remove unnecessary `closure` usage
- add missing base errors for retriever error
- **url**: add missing url parser
- **async**: add `async` to functions
- string format with `clippy` suggestions
- forbidden 403 problem

### Refactor

- add `impl` for `TranslationResult`
- rename generic retriever error
- export html parsing to function
