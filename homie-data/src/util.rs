use homie_core::adapter::importer::Importer;
use homie_core::adapter::repository::Repository;
use homie_core::error::DomainError;

pub(crate) async fn read_and_write_datasets(
    importer: &Importer,
    repo: &Repository,
) -> Result<(), DomainError> {
    read_and_write_t_yields(importer, repo).await?;
    read_and_write_hpi(importer, repo).await?;
    read_and_write_region(importer, repo).await?;
    read_and_write_zhvi(importer, repo).await?;

    Ok(())
}

async fn read_and_write_t_yields(
    importer: &Importer,
    repo: &Repository,
) -> Result<(), DomainError> {
    let t_yield_data = importer.read_fed_yields()?;
    for t_yield in t_yield_data.ten_year_yields() {
        t_yield.create(repo.session()).await?;
    }
    Ok(())
}

async fn read_and_write_hpi(importer: &Importer, repo: &Repository) -> Result<(), DomainError> {
    let hpi_data = importer.read_fhfa_hpis()?;

    for hpi in hpi_data.three_zip_hpis() {
        hpi.create(repo.session()).await?;
    }

    for hpi in hpi_data.five_zip_hpis() {
        hpi.create(repo.session()).await?;
    }

    for hpi in hpi_data.county_hpis() {
        hpi.create(repo.session()).await?;
    }

    Ok(())
}

async fn read_and_write_region(importer: &Importer, repo: &Repository) -> Result<(), DomainError> {
    let region_data = importer.read_huduser_regions()?;
    for region in region_data.regions() {
        region.create(repo.session()).await?;
    }
    Ok(())
}

async fn read_and_write_zhvi(importer: &Importer, repo: &Repository) -> Result<(), DomainError> {
    let zhvi_data = importer.read_zillow_zhvis()?;

    for zhvi in zhvi_data.all_homes_zhvis() {
        zhvi.create(repo.session()).await?;
    }

    for zhvi in zhvi_data.condo_coops_zhvis() {
        zhvi.create(repo.session()).await?;
    }

    for zhvi in zhvi_data.single_family_homes_zhvis() {
        zhvi.create(repo.session()).await?;
    }
    Ok(())
}
