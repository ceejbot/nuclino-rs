# nuclino-rs

This is a Rust client for the [Nuclino API](https://help.nuclino.com/d3a29686-api). Simplicity and rapid development were my goals for this client, so it uses [ureq](https://lib.rs/crates/ureq) as its base http client. This means all api requests are blocking. If this client would be more useful to you with an async http client, let me know.

## Usage

Create an [API key](https://help.nuclino.com/04598850-manage-api-keys) for Nuclino. Provide it in the env var `NUCLINO_API_KEY` or pass this to the client `create()` function. `cargo add nuclino-rs` in your project and then start making requests using the client's functions.

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

- handle rate limiting
- break up the `types.rs` file into related sections
- tidy up the `ureq` http method wrappers
- write doc strings for everything
- publish the crate

## LICENSE

This code is licensed via [the Parity Public License.](https://paritylicense.com) This license requires people who build on top of this source code to share their work with the community, too. See the license text for details.
