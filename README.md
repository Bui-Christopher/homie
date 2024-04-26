# homie 🏠📊 - Home Insight Extraction <span style="float:right;"> ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Bui-Christopher/homie)</span>
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Zillow](https://img.shields.io/badge/Zillow-006AFF.svg?style=for-the-badge&logo=Zillow&logoColor=white) ![PostgreSQL](https://img.shields.io/badge/postgresql-4169e1?style=for-the-badge&logo=postgresql&logoColor=white) ![Leptos](https://img.shields.io/badge/Leptos-EF3939.svg?style=for-the-badge&logo=Leptos&logoColor=white) ![Plotly](https://img.shields.io/badge/Plotly-3F4F75.svg?style=for-the-badge&logo=Plotly&logoColor=white)

## Description 💡
Homie aims to display visualizations of housing market datasets.

It answers two vital questions:
- What value of a home is fair? 💰
- What value of a mortgage interest rate is fair? 📈

Current datasets include Zillow, FHFA, Fed Treasury, and Huduser.
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
- [ ] homie-api
    - [x] Handle Request
    - [ ] Retrieve Data
    - [ ] Return Response
- [ ] homie-webapp
    - [ ] Get Query Params
    - [ ] Submit Request
    - [ ] Display Data in Graph
- [ ] Unit tests :sob:
### Current Bugs 🐜
- [ ] Parsing CSV
    - [ ] Zillow Regions
    - [x] Fed H15 (daily) has some null values


#### Additional Notes
- [ ] homie-api
    - [x] Mock data
    - [ ] Read from DB (Actual Dataset)
- [ ] homie-webapp
    - [ ] Leptos
    - [ ] Plotly
    - [ ] Use Axum
```
NOTE: Will only write zillow-mid/city to local db (downloaded from zillow/datasets)
```

- [x] Read remaining datasets
    - [x] AllHomes
        - [x] Bottom-tier
            - [x] City
            - [x] County
        - [ ] Mid-tier
            - [ ] City
        - [x] Top-tier
            - [x] City
            - [x] County
    - [x] CondoCoops
        - [x] City
        - [x] County
        - [x] Zip
    - [x] SingleFamilyHomes
        - [x] City
        - [x] County
        - [x] Zip
- [ ] Write datasets to DB (postgreSQL)
    - [ ] fed_h15
    - [ ] fhfa-hpi
    - [ ] cities/counties/zipcodes
    - [ ] zillow-zhvi
    - [x] ~Split between postgres and in memory~

- [ ] homie-core
    - [x] Refactor common code to `homie-core` (Previously logic was in each binary)
    - [ ] Double check what structs/mods are exposed may limit it to the crate
    - [ ] Read Software Architecture
        - [x] Dependency Inversion Principle
        - [x] Singleton vs Dependency Injection
        - [x] Test some patterns
            - [x] Factory
            - [ ] Builder
            - [x] Singleton
            - [x] Mediator
            - [x] Facade
        - [x] CQRS
        - [x] High Level Patterns (Not application level?)
            - [x] Model-View-Controller (MVC)
            - [x] Service-Oriented Architecture (SOA)
            - [x] Event-Driven Architecture (EDA)
- [ ] Deployment
    - [ ] Local Deployment
    - [ ] CI

## Additional Features 🌕
```
NOTE: Not necessary for minimum viable product, there's more listed on the excalidraw.
```
- [ ] Expand dataset scope outside of Orange County
