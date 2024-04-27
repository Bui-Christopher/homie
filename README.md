# homie 🏠📊 - Home Insight Extraction <span style="float:right;"> ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Bui-Christopher/homie)</span>
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Zillow](https://img.shields.io/badge/Zillow-006AFF.svg?style=for-the-badge&logo=Zillow&logoColor=white) ![PostgreSQL](https://img.shields.io/badge/postgresql-4169e1?style=for-the-badge&logo=postgresql&logoColor=white) ![Leptos](https://img.shields.io/badge/Leptos-EF3939.svg?style=for-the-badge&logo=Leptos&logoColor=white) ![Plotly](https://img.shields.io/badge/Plotly-3F4F75.svg?style=for-the-badge&logo=Plotly&logoColor=white)

## Description 💡
Homie aims to display visualizations of housing market datasets.

It answers two vital questions:
- What value of a home is fair? 💰
- What value of a mortgage interest rate is fair? 📈

Current datasets include [Zillow](https://www.zillow.com/research/data/), [FHFA](https://www.fhfa.gov/DataTools/Downloads/Pages/House-Price-Index-Datasets.aspx), [Fed Treasury](https://www.federalreserve.gov/releases/h15/), and Huduser.
<!-- TODO: Link each dataset -->

## Quick Start ⚡
```
TODO: Local scrpts to run and test
```

## TODOs 📋
### General Outline
- [x] homie-core
    - [x] Define Domain
    - [x] Define Adapters
- [ ] homie-data
    - [x] Read Dataset
    - [x] Convert to Application Domain
    - [ ] Store into repository (Postgres)
        - [ ] Regions (cities/counties/zipcodes)
    - [ ] Optimize ZHVI (batch insert prices)
- [ ] homie-api
    - [x] Handle Request
    - [ ] Retrieve Data
        - [ ] Postgres
        - [ ] Zillow API
    - [ ] Return Response
        - [ ] Error handling
            - [ ] Custom
            - [ ] ThisError and Anyhow
- [ ] homie-webapp
    - [ ] Get Query Params
    - [ ] Submit Request
    - [ ] Display Data in Graph
- [ ] Unit tests :sob:

#### Additional Notes
- [ ] Reduce public struct/fn exposure
- [ ] Read Bulder Pattern
- [ ] Deployment
    - [ ] Local Deployment
    - [ ] CI
