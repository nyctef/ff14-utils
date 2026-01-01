use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// Include shared type definitions used for both serialization and deserialization
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded_types.rs"));

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    // Look for ffxiv-datamining repository
    let datamining_csv = if let Ok(nix_path) = env::var("FFXIV_DATAMINING_PATH") {
        // flake.nix sets it for nix builds
        Path::new(&nix_path).join("csv")
    } else {
        // or we expect it to be checked out next to this repo for local builds
        Path::new(&manifest_dir)
            .parent()
            .unwrap()
            .parent()
            .expect("Failed to get parent directory")
            .join("ffxiv-datamining")
            .join("csv")
    };

    if !datamining_csv.exists() {
        panic!(
            "ffxiv-datamining repository not found. Expected at: {:?}\n\
            For local development, clone the repository:\n\
            cd {}\n\
            git clone https://github.com/xivapi/ffxiv-datamining.git",
            datamining_csv,
            Path::new(&manifest_dir).parent().unwrap().display()
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("Parsing CSV files...");

    // Parse all CSV files
    let items = parse_items(&datamining_csv).unwrap();
    let recipes = parse_recipes(&datamining_csv).unwrap();
    let recipe_levels = parse_recipe_levels(&datamining_csv).unwrap();
    let materia = parse_materia(&datamining_csv).unwrap();

    println!(
        "Parsed {} items, {} recipes, {} recipe levels, {} materia",
        items.len(),
        recipes.len(),
        recipe_levels.len(),
        materia.len()
    );

    let data = EmbeddedData {
        items,
        recipes,
        recipe_levels,
        materia,
    };

    println!("Serializing data with rkyv...");
    let bytes = rkyv::to_bytes::<_, 256>(&data).unwrap();
    println!("Serialized {} bytes ({:.2} MB)", bytes.len(), bytes.len() as f64 / 1_000_000.0);

    let data_path = Path::new(&out_dir).join("embedded_data.bin");
    std::fs::write(&data_path, &bytes).unwrap();
    println!("✓ Wrote embedded data to {:?}", data_path);

    println!("Generating PHF lookup maps...");
    let code_path = Path::new(&out_dir).join("generated_lookups.rs");
    let mut file = BufWriter::new(File::create(&code_path).unwrap());

    generate_lookups(&mut file, &data).unwrap();

    println!("✓ Wrote lookups to {:?}", code_path);
}

fn generate_lookups(file: &mut BufWriter<File>, data: &EmbeddedData) -> std::io::Result<()> {
    writeln!(file, "use phf::Map;")?;
    writeln!(file)?;

    // item ID -> index
    let mut id_map = phf_codegen::Map::new();
    for (idx, item) in data.items.iter().enumerate() {
        id_map.entry(item.id, idx.to_string());
    }
    writeln!(file, "pub static ITEM_ID_TO_INDEX: Map<i32, usize> = {};\n", id_map.build())?;

    // item name -> index
    // Note: Skip duplicates since some items share names
    let mut seen_names = std::collections::HashSet::new();
    let mut name_map = phf_codegen::Map::new();
    for (idx, item) in data.items.iter().enumerate() {
        if !item.name.is_empty() && seen_names.insert(&item.name) {
            name_map.entry(&item.name, idx.to_string());
        }
    }
    writeln!(file, "pub static ITEM_NAME_TO_INDEX: Map<&'static str, usize> = {};\n", name_map.build())?;

    // recipe ID -> index
    let mut recipe_map = phf_codegen::Map::new();
    for (idx, recipe) in data.recipes.iter().enumerate() {
        recipe_map.entry(recipe.id, idx.to_string());
    }
    writeln!(file, "pub static RECIPE_ID_TO_INDEX: Map<i32, usize> = {};\n", recipe_map.build())?;

    // recipe result item -> recipe index
    // Note: Skip duplicates since multiple recipes can produce the same item (keeps first)
    let mut seen_results = std::collections::HashSet::new();
    let mut result_map = phf_codegen::Map::new();
    for (idx, recipe) in data.recipes.iter().enumerate() {
        if seen_results.insert(recipe.item_result) {
            result_map.entry(recipe.item_result, idx.to_string());
        }
    }
    writeln!(file, "pub static RECIPE_RESULT_TO_INDEX: Map<i32, usize> = {};\n", result_map.build())?;

    // recipe level ID -> index
    let mut rlvl_map = phf_codegen::Map::new();
    for (idx, rlvl) in data.recipe_levels.iter().enumerate() {
        rlvl_map.entry(rlvl.id, idx.to_string());
    }
    writeln!(file, "pub static RECIPE_LEVEL_ID_TO_INDEX: Map<i32, usize> = {};\n", rlvl_map.build())?;

    // Include the binary data
    writeln!(file, "pub static EMBEDDED_DATA_BYTES: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/embedded_data.bin\"));")?;

    Ok(())
}

fn parse_items(csv_dir: &Path) -> Result<Vec<ItemRow>, Box<dyn std::error::Error>> {
    let csv_path = csv_dir.join("Item.csv");
    let (headers, records) = parse_csv_file(&csv_path)?;

    let mut items = Vec::new();
    for record in records {
        let id: i32 = get_field(&record, &headers, "#")?.parse()?;
        if id == 0 {
            continue;
        }

        items.push(ItemRow {
            id,
            name: get_field(&record, &headers, "Name")?.to_string(),
            singular: get_field(&record, &headers, "Singular")?.to_string(),
            plural: get_field(&record, &headers, "Plural")?.to_string(),
            ilvl: get_field(&record, &headers, "Level{Item}")?.parse().unwrap_or(0),
            can_be_hq: get_field(&record, &headers, "CanBeHq")? == "True",
            equip_slot: get_field(&record, &headers, "EquipSlotCategory")?.parse().unwrap_or(0),
        });
    }

    Ok(items)
}

fn parse_recipes(csv_dir: &Path) -> Result<Vec<RecipeRow>, Box<dyn std::error::Error>> {
    let csv_path = csv_dir.join("Recipe.csv");
    let (headers, records) = parse_csv_file(&csv_path)?;

    let mut recipes = Vec::new();
    for record in records {
        let id: i32 = get_field(&record, &headers, "#")?.parse()?;
        if id == 0 {
            continue;
        }

        let mut item_ingredients = Vec::new();
        let mut amount_ingredients = Vec::new();
        for i in 0..8 {
            let item_id: i32 = get_field(&record, &headers, &format!("Item{{Ingredient}}[{}]", i))?.parse().unwrap_or(0);
            let amount: u32 = get_field(&record, &headers, &format!("Amount{{Ingredient}}[{}]", i))?.parse().unwrap_or(0);
            item_ingredients.push(item_id);
            amount_ingredients.push(amount);
        }

        recipes.push(RecipeRow {
            id,
            number: get_field(&record, &headers, "Number")?.parse().unwrap_or(0),
            craft_type: get_field(&record, &headers, "CraftType")?.parse().unwrap_or(0),
            recipe_level_table: get_field(&record, &headers, "RecipeLevelTable")?.parse().unwrap_or(0),
            item_result: get_field(&record, &headers, "Item{Result}")?.parse().unwrap_or(0),
            amount_result: get_field(&record, &headers, "Amount{Result}")?.parse().unwrap_or(0),
            item_ingredients,
            amount_ingredients,
            difficulty_factor: get_field(&record, &headers, "DifficultyFactor")?.parse().unwrap_or(0),
            quality_factor: get_field(&record, &headers, "QualityFactor")?.parse().unwrap_or(0),
            durability_factor: get_field(&record, &headers, "DurabilityFactor")?.parse().unwrap_or(0),
            required_craftsmanship: get_field(&record, &headers, "RequiredCraftsmanship")?.parse().unwrap_or(0),
            required_control: get_field(&record, &headers, "RequiredControl")?.parse().unwrap_or(0),
        });
    }

    Ok(recipes)
}

fn parse_recipe_levels(csv_dir: &Path) -> Result<Vec<RecipeLevelRow>, Box<dyn std::error::Error>> {
    let csv_path = csv_dir.join("RecipeLevelTable.csv");
    let (headers, records) = parse_csv_file(&csv_path)?;

    let mut levels = Vec::new();
    for record in records {
        let id: i32 = get_field(&record, &headers, "#")?.parse()?;

        levels.push(RecipeLevelRow {
            id,
            progress_divider: get_field(&record, &headers, "ProgressDivider")?.parse().unwrap_or(0),
            progress_modifier: get_field(&record, &headers, "ProgressModifier")?.parse().unwrap_or(0),
            quality_divider: get_field(&record, &headers, "QualityDivider")?.parse().unwrap_or(0),
            quality_modifier: get_field(&record, &headers, "QualityModifier")?.parse().unwrap_or(0),
            difficulty: get_field(&record, &headers, "Difficulty")?.parse().unwrap_or(0),
            durability: get_field(&record, &headers, "Durability")?.parse().unwrap_or(0),
            quality: get_field(&record, &headers, "Quality")?.parse().unwrap_or(0),
            stars: get_field(&record, &headers, "Stars")?.parse().unwrap_or(0),
        });
    }

    Ok(levels)
}

fn parse_materia(csv_dir: &Path) -> Result<Vec<MateriaRow>, Box<dyn std::error::Error>> {
    let csv_path = csv_dir.join("Materia.csv");
    let (headers, records) = parse_csv_file(&csv_path)?;

    let mut materia = Vec::new();
    for record in records {
        let id: i32 = get_field(&record, &headers, "#")?.parse()?;
        if id == 0 {
            continue;
        }

        let mut item_ids = Vec::new();
        let mut values = Vec::new();
        for i in 0..12 {
            let item_id: i32 = get_field(&record, &headers, &format!("Item[{}]", i))?.parse().unwrap_or(0);
            let value: i16 = get_field(&record, &headers, &format!("Value[{}]", i))?.parse().unwrap_or(0);
            item_ids.push(item_id);
            values.push(value);
        }

        materia.push(MateriaRow {
            id,
            item_ids,
            values,
        });
    }

    Ok(materia)
}

fn get_field<'a>(
    record: &'a [String],
    headers: &[String],
    field_name: &str,
) -> Result<&'a str, Box<dyn std::error::Error>> {
    let index = headers
        .iter()
        .position(|h| h == field_name)
        .ok_or_else(|| format!("Field '{}' not found in headers", field_name))?;
    Ok(record.get(index).map(|s| s.as_str()).unwrap_or(""))
}

fn parse_csv_file(
    path: &Path,
) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    let mut raw_records = reader.records();

    // Skip first row (column indices)
    raw_records.next();

    // Read second row (field names)
    let header_record = raw_records
        .next()
        .transpose()?
        .ok_or("Missing header row")?;
    let headers: Vec<String> = header_record.iter().map(|s| s.to_string()).collect();

    // Skip third row (data types)
    raw_records.next();

    // Parse data rows
    let mut records = Vec::new();
    for record in raw_records {
        let record = record?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        records.push(row);
    }

    Ok((headers, records))
}
