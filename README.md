# homie 🏠📊 - Home Insight Extraction <span style="float:right;"> ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Bui-Christopher/homie)</span>
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Zillow](https://img.shields.io/badge/Zillow-006AFF.svg?style=for-the-badge&logo=Zillow&logoColor=white) ![PostgreSQL](https://img.shields.io/badge/postgresql-4169e1?style=for-the-badge&logo=postgresql&logoColor=white) ![Leptos](https://img.shields.io/badge/Leptos-EF3939.svg?style=for-the-badge&logo=Leptos&logoColor=white) ![Plotly](https://img.shields.io/badge/Plotly-3F4F75.svg?style=for-the-badge&logo=Plotly&logoColor=white)

## Description 💡
Homie aims to display visualizations of housing market datasets.

It answers two vital questions:
- What value of a home is fair? 💰
- What value of a mortgage interest rate is fair? 📈

Current datasets include [Zillow](https://www.zillow.com/research/data/), [FHFA](https://www.fhfa.gov/DataTools/Downloads/Pages/House-Price-Index-Datasets.aspx), [Fed Treasury](https://www.federalreserve.gov/releases/h15/), and Huduser.

## Quick Start ⚡
### Requirements
Before getting started, make sure you have Docker installed on your system.

### Running
```
./local/run.sh
```
This script will pull and run the required images. It will locally deploy with a database, backend, and frontend.
Please check http://localhost:3000.

## MVP TODOs 📋
- [x] homie-core
    - [x] Define Domain
    - [x] Define Adapters
- [ ] homie-data
    - [x] Read Dataset
    - [x] Convert to Application Domain
    - [ ] Store into repository (Postgres)
        - [ ] Regions (cities/counties/zipcodes)
- [x] homie-api
    - [x] Handle Request
    - [x] Retrieve Data
    - [x] Return Response
- [ ] homie-webapp
    - [ ] Get User's Query Params
    - [ ] Submit Request
    - [ ] Display Data in Graph

## Improvements
- [x] Local Development
    - [x] Automate creating database
        - [x] Can also initialize with `docker exec`
    - [x] Reduce local datasets size
        - [x] HPI
        - [x] Region
- [ ] Optimize ZHVI (batch insert prices)
- [x] Refactor
    - [x] Reduce public struct/fn exposure
    - [x] ~Read Bulder Pattern~ Won't use it, but I understand it
    - [x] Enums instead of Strings [Example](https://github.com/launchbadge/sqlx/discussions/3041)
    - [x] Repo/Import calls into Config
- [ ] Error Handling
    - [x] ~ThisError and Anyhow~ Created a custom enum instead
    - [x] Logging
- [ ] Deployment
    - [x] [Dockerize](https://itnext.io/a-practical-guide-to-containerize-your-rust-application-with-docker-77e8a391b4a8)
    - [x] ~Push to dockerhub~ Going to use bash script to build and deploy
    - [ ] Read Zhvi from Zillow API
- [ ] Testing 
    - [ ] Unit tests :sob:
    - [ ] E2E tests :sob:
