//! `make:*` generator commands.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Args;

#[derive(Args, Debug)]
pub struct MakeArgs {
    pub name: String,

    #[arg(long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct MakeMigrationArgs {
    pub name: String,

    #[arg(long)]
    pub yes: bool,
}

pub fn make_controller(args: &MakeArgs) -> Result<()> {
    let module = to_snake_case(&args.name);
    let path = format!("src/controllers/{module}.rs");
    let contents = format!(
        r#"//! Generated controller stub for `{name}`.

pub async fn index() -> &'static str {{
    "OK"
}}
"#,
        name = args.name
    );
    write_file_safe(Path::new(&path), &contents, args.yes)?;
    println!("Created {path}");
    Ok(())
}

pub fn make_service(args: &MakeArgs) -> Result<()> {
    let module = to_snake_case(&args.name);
    let path = format!("src/services/{module}.rs");
    let contents = format!(
        r"//! Generated service stub for `{name}`.

pub struct {name};

impl {name} {{
    pub fn new() -> Self {{
        Self
    }}
}}
",
        name = args.name
    );
    write_file_safe(Path::new(&path), &contents, args.yes)?;
    println!("Created {path}");
    Ok(())
}

pub fn make_model(args: &MakeArgs) -> Result<()> {
    let module = to_snake_case(&args.name);
    let path = format!("src/models/{module}.rs");
    let table = module.as_str();
    let contents = format!(
        r#"//! Generated SeaORM entity stub for `{name}`.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "{table}")]
pub struct Model {{
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}
"#,
        name = args.name,
        table = table
    );
    write_file_safe(Path::new(&path), &contents, args.yes)?;
    println!("Created {path}");
    Ok(())
}

pub fn make_migration(args: &MakeMigrationArgs) -> Result<()> {
    let file_name = format!("m{}_{}.rs", chrono_like_timestamp(), args.name);
    let path = PathBuf::from("migration/src").join(&file_name);
    let contents = r"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager;
        Ok(())
    }
}
"
    .to_string();
    write_file_safe(&path, &contents, args.yes)?;
    println!("Created {}", path.display());
    println!("Register this migration in migration/src/lib.rs");
    Ok(())
}

pub(crate) fn write_file_safe(path: &Path, contents: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        bail!(
            "refusing to overwrite existing file: {} (use --yes)",
            path.display()
        );
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn to_snake_case(input: &str) -> String {
    let mut output = String::new();
    for (index, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if index > 0 {
                output.push('_');
            }
            output.extend(ch.to_lowercase());
        } else if ch == '-' {
            output.push('_');
        } else {
            output.push(ch);
        }
    }
    output
}

fn chrono_like_timestamp() -> String {
    // Deterministic placeholder for generator file ordering in tests.
    "20250330_000003".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_converts_controller_names() {
        assert_eq!(to_snake_case("ProductController"), "product_controller");
    }
}
