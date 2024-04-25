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
    let mut t_yield_data = importer.read_fed_yields()?;
    // TODO: Remove. Does not have to be mut here
    for t_yield in t_yield_data.ten_year_yields_mut() {
        t_yield.create(repo.session()).await?;

        // TODO: Delete (testing all CRUD for t_yield)
        // let start_date = chrono::NaiveDate::from_ymd_opt(2000, 1,
        // 1).unwrap(); let end_date =
        // chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        // let interval_date = "Month".to_string();
        // let query =
        //     homie_core::domain::t_yield::TYieldQuery::new(start_date,
        // end_date, interval_date); let t_yields =
        //     homie_core::domain::t_yield::TYield::read_by_query(repo.
        // session(), &query).await?; println!("{:?}", t_yields);
        // t_yield.set_yield_return(None);
        // t_yield.update(repo.session()).await?;
        // let test = homie_core::domain::t_yield::TYield::read(
        //     repo.session(),
        //     (t_yield.term(), t_yield.date()),
        // )
        // .await?;
        // homie_core::domain::t_yield::TYield::delete(
        //     repo.session(),
        //     (t_yield.term(), t_yield.date()),
        // )
        // .await?;
        // println!("{:?}", test);
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
