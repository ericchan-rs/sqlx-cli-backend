use std::{fs, path::Path};

use walkdir::WalkDir;

fn main() {
    let mut all_lang = Vec::<String>::new();

    // load i18n/<language>.toml
    let walk_dir = WalkDir::new("./languages");
    for entry in walk_dir {
        let entry = entry.unwrap();
        if !entry.path().is_file() {
            continue;
        }

        // '"en.toml"'
        let origin_language_file_name = entry.path().file_name().unwrap().to_str().unwrap();
        // 'en' or 'zh_CN'
        let lang_file_name = origin_language_file_name
            .replace(".toml", "")
            .replace("\"", "");
        // 'en' or 'zh_cn'
        let lang_file_name_lowercase = lang_file_name.to_lowercase();

        // src/lang.rs start
        all_lang.push(format!(
            "    let {}_toml = include_str!(\"../languages/{}.toml\");",
            lang_file_name_lowercase, lang_file_name
        ));
        all_lang.push(format!(
            "    let {}_language: HashMap<&str, &str> = toml::from_str(&{}_toml).unwrap();",
            lang_file_name_lowercase, lang_file_name_lowercase
        ));
        all_lang.push(format!(
            "    all.insert(\"{}\", {}_language);",
            lang_file_name, lang_file_name_lowercase
        ));
        all_lang.push("".to_string());
        // src/lang.rs end
    }

    // src/lang.rs
    let mut all_c = Vec::new();
    all_c.push("use std::collections::HashMap;");
    all_c.push("");
    all_c.push("use once_cell::sync::Lazy;");
    all_c.push("");
    all_c.push("pub(crate) static ALL_LANGUAGES: Lazy<HashMap<&str, HashMap<&str, &str>>> = Lazy::new(|| {");
    all_c.push("    let mut all = HashMap::<&str, HashMap<&str, &str>>::new();");
    all_c.push("");
    let all_lang = &all_lang.join("\n");
    all_c.push(all_lang);
    all_c.push("    all");
    all_c.push("});");
    let dest_path = Path::new("./src").join("lang.rs");
    fs::write(&dest_path, all_c.join("\n")).unwrap();
}