use fluent_resmgr::resource_manager::ResourceManager;

fn main() {
    let mgr = ResourceManager::new("./i18n/languages/{locale}/{res_id}.ftl".into());
    let supported_locale_identifiers = vec![
        "en-US",
    ];
    let supported_locales = supported_locale_identifiers.into_iter().map(|id| {
        id.parse().expect("Could not parse locale Identifier")
    }).collect();
    let resources = vec![
        String::from("common")
    ];

    let bundle = mgr.get_bundle(supported_locales, resources);
    let mut errors = vec![];
    let hello_world_message = bundle.get_message("hello-world").unwrap();
    let hello_world_pattern = hello_world_message.value.unwrap();
    println!("{}", bundle.format_pattern(&hello_world_pattern, None, &mut errors));
}
