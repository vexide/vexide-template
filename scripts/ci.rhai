if variable::get("use_ci") {
    debug("Generating GitHub CI configuration...");
    file::write(".github/workflows/build.yml", `
name: Build

on: [push, pull_request]

jobs:
    check:
        name: Check
        runs-on: ubuntu-latest
        steps:
            - name: Setup | Checkout
              uses: actions/checkout@v2

            - name: Setup | Toolchain
              uses: dtolnay/rust-toolchain@master
              with:
                  toolchain: nightly-2024-02-07
                  components: rust-src

            - name: Check
              uses: actions-rs/cargo@v1
              with:
                  command: check
`);
} else {
    debug("Skipping GitHub CI configuration...");
    file::delete(".github");
}
