use std::error::Error;

use homie_core::adapter::importer::Importer;
use homie_core::adapter::repository::Repository;

pub(crate) async fn read_and_write_datasets(
    importer: &Importer,
    repo: &Repository,
) -> Result<(), Box<dyn Error>> {
    read_and_write_t_yields(importer, repo).await?;
    read_and_write_hpi(importer, repo)?;
    read_and_write_region(importer, repo)?;
    read_and_write_zhvi(importer, repo)?;

    Ok(())
}

async fn read_and_write_t_yields(
    importer: &Importer,
    repo: &Repository,
) -> Result<(), Box<dyn Error>> {
    let t_yield_data = importer.read_fed_yields()?;
    for t_yield in t_yield_data.ten_year_yields() {
        t_yield.create(repo.session()).await?;
    }
    Ok(())
}

fn read_and_write_hpi(importer: &Importer, repo: &Repository) -> Result<(), Box<dyn Error>> {
    let hpi_data = importer.read_fhfa_hpis()?;

    for hpi in hpi_data.three_zip_hpis() {
        hpi.create(repo.session())?;
    }

    for hpi in hpi_data.five_zip_hpis() {
        hpi.create(repo.session())?;
    }

    for hpi in hpi_data.county_hpis() {
        hpi.create(repo.session())?;
    }

    Ok(())
}

fn read_and_write_region(_importer: &Importer, _repo: &Repository) -> Result<(), Box<dyn Error>> {
    // TODO: Handle what zipcodes/cities will be stored and its mappings
    // let region_data = importer.read_huduser_regions()?;
    // for t_yield in t_yield_data
    //     t_yield.create(repo)
    Ok(())
}

fn read_and_write_zhvi(importer: &Importer, repo: &Repository) -> Result<(), Box<dyn Error>> {
    let zhvi_data = importer.read_zillow_zhvis()?;

    for zhvi in zhvi_data.all_homes_zhvis() {
        zhvi.create(repo.session())?;
    }

    for zhvi in zhvi_data.condo_coops_zhvis() {
        zhvi.create(repo.session())?;
    }

    for zhvi in zhvi_data.single_family_homes_zhvis() {
        zhvi.create(repo.session())?;
    }
    Ok(())
}
