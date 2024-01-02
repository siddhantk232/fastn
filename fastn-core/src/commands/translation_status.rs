pub async fn translation_status(config: &fastn_core::Config) -> fastn_core::Result<()> {
    // it can be original package or translation
    if config.is_translation_package() {
        translation_package_status(config).await?;
    } else if !config.package.translations.is_empty() {
        original_package_status(config).await?;
    } else {
        return Err(fastn_core::Error::UsageError {
            message:
                "`translation-status` works only when either `translation` or `translation-of` is set."
                    .to_string(),
        });
    };
    Ok(())
}

async fn translation_package_status(config: &fastn_core::Config) -> fastn_core::Result<()> {
    let original_snapshots =
        fastn_core::snapshot::get_latest_snapshots(&config.original_path()?).await?;
    let translation_status =
        get_translation_status(config, &original_snapshots, &config.ds.root()).await?;
    print_translation_status(&translation_status);
    Ok(())
}

async fn original_package_status(config: &fastn_core::Config) -> fastn_core::Result<()> {
    for translation in config.package.translations.iter() {
        if let Some(ref status) = translation.translation_status_summary {
            println!("Status for `{}` package:", translation.name);
            println!("{}", status.to_string());
        }
    }
    Ok(())
}

pub(crate) async fn get_translation_status(
    config: &fastn_core::Config,
    snapshots: &std::collections::BTreeMap<String, u128>,
    path: &camino::Utf8PathBuf,
) -> fastn_core::Result<std::collections::BTreeMap<String, TranslationStatus>> {
    let mut translation_status = std::collections::BTreeMap::new();
    for (file, timestamp) in snapshots {
        if !path.join(file).exists() {
            translation_status.insert(file.clone(), TranslationStatus::Missing);
            continue;
        }
        let track_path = fastn_core::utils::track_path(file.as_str(), path.as_str());
        if !track_path.exists() {
            translation_status.insert(file.clone(), TranslationStatus::NeverMarked);
            continue;
        }
        let tracks = fastn_core::tracker::get_tracks(config, path.as_str(), &track_path).await?;
        if let Some(fastn_core::Track {
            last_merged_version: Some(last_merged_version),
            ..
        }) = tracks.get(file)
        {
            if last_merged_version < timestamp {
                translation_status.insert(file.clone(), TranslationStatus::Outdated);
                continue;
            }
            translation_status.insert(file.clone(), TranslationStatus::UptoDate);
        } else {
            translation_status.insert(file.clone(), TranslationStatus::NeverMarked);
        }
    }
    Ok(translation_status)
}

fn print_translation_status(
    translation_status: &std::collections::BTreeMap<String, TranslationStatus>,
) {
    for (file, status) in translation_status {
        println!("{}: {}", status.as_str(), file);
    }
}

pub(crate) enum TranslationStatus {
    Missing,
    NeverMarked,
    Outdated,
    UptoDate,
}

impl TranslationStatus {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            TranslationStatus::Missing => "Missing",
            TranslationStatus::NeverMarked => "Never marked",
            TranslationStatus::Outdated => "Out-dated",
            TranslationStatus::UptoDate => "Up to date",
        }
    }
}
