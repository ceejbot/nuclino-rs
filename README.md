# nuclino-rs

[![Tests](https://github.com/ceejbot/nuclino-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/ceejbot/nuclino-rs/actions/workflows/test.yaml)

This is a Rust client for the [Nuclino API](https://help.nuclino.com/d3a29686-api). Simplicity and rapid development were my goals for this project and the project that I need it for, so it uses [ureq](https://lib.rs/crates/ureq) as its base http client. This means all api requests are blocking. If this client would be more useful to you with an async http client, let me know.


## Usage

 `cargo add nuclino-rs` in your project to add the library. There are no optional features. Create an [API key](https://help.nuclino.com/04598850-manage-api-keys) for Nuclino. Provide it in the env var `NUCLINO_API_KEY` and call `nuclino_rs::Client::create_from_env()` to create a default client. Or you can provide it to your program in some other way and pass it to the client `create()` function.and then start making requests using the client's functions.

 `cargo doc --open` has more information.

## Example

```rs
let client = nuclino_rs::Client::create_from_env()?;
let workspaces = client.workspace_list(None, None)?.to_vec();
let first = workspaces.first().unwrap();

let newpage = nuclino_rs::NewPageBuilder::item()
    .title("I'm just a test")
    .content(
        "Yes I'm only a *test* and I'm sitting here on a Capitol Hill. Wait. That didn't rhyme.",
    )
    .workspace(first.id())
    .build();
let newpage = client.page_create(newpage)?;
```

See `examples/iterate_workspace_pages.rs` for a more complex example of accessing Nuclino data, creating wiki pages, and deleting them. `cargo run --example iterate_workspace_pages` to run this example.

## TODO

The API should be completely covered and theoretically working. The parts I've needed to use for my project are definitely working. Known work that I'd like to do:

- handle rate limiting
- break up the `types.rs` file into related sections
- publish the crate

## LICENSE

This code is licensed via [the Parity Public License.](https://paritylicense.com) This license requires people who build on top of this source code to share their work with the community, too. See the license text for details.
