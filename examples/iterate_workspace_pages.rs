use nuclino_rs::NuclinoError;
use nuclino_rs::Page;
use owo_colors::OwoColorize;

fn main() -> Result<(), NuclinoError> {
    if dotenvy::dotenv().is_err() {
        println!(
            "no .env file found; you should already have {} set in your env",
            nuclino_rs::APIKEY_ENV_VAR
        );
    }

    let client = nuclino_rs::Client::create_from_env()?;
    let workspaces = client.workspace_list(None, None)?.to_vec();

    println!("Nuclino workspaces you have access to:");
    workspaces.iter().for_each(|space| {
        println!("    {}: {}", space.name().blue(), space.id());
    });

    let maybe_eng = workspaces.iter().find(|xs| xs.name() == "Engineering");
    if let Some(eng) = maybe_eng {
        println!(
            "\nThe workspace named '{}' has {} direct children. Fetching...",
            "Engineering".blue(),
            eng.children().len()
        );

        let _pages: Vec<Page> = eng
            .children()
            .iter()
            .filter_map(|id| match client.page(id) {
                Ok(page) => {
                    let id = page.id();
                    let pagekind = match page {
                        Page::Item(_) => "item",
                        Page::Collection(_) => "collection",
                    };
                    println!("    {}: {} {id}", page.title().yellow(), pagekind);
                    Some(page)
                }
                Err(_) => None,
            })
            .collect();
        // do something else with pages to exercise the client, I guess

        let newpage = nuclino_rs::NewPageBuilder::item()
            .title("This is a test")
            .content(
                "Yes it's only a *test* and I'm sitting here on a Capitol Hill. Wait. Wrong song.",
            )
            .workspace(eng.id())
            .build();
        let created = client.page_create(newpage)?;
        println!("created new page at {}", created.url().yellow());

        let _deleted = client.page_delete(created.id())?;
        println!("Moved the page to the trash. Probably. Go check!");
    }

    Ok(())
}
