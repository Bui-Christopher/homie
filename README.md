# homie - Home Insight Extraction
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Bui-Christopher/homie/main)

![CI](https://img.shields.io/badge/GitHub_Actions-2088FF?style=for-the-badge&logo=github-actions&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
<!-- Add postgreSQL icon -->
<!-- Add Zillow icon -->

## Description
Homie aims to display visual representations of datasets related to the housing market. It answers two vital questions:
- What value of a home is fair?
- What value of a mortgage interest rate is fair?

Current datasets include Zillow, FHFA, Fed Treasury, and Huduser.

## Bugs
- [ ] Parsing CSV
    - [ ] Zillow Regions
    - [ ] Download Fed H15 daily datset has some null values

## TODO
- [ ] Create HTTP API
    - [x] Mock data
    - [ ] Read from DB (Actual Dataset)
- [ ] Leptos
- [ ] Plotly
- [ ] Connect to HTTP API
`Will only write zillow/mid/city to local db (downloaded from zillow/datasets)`
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
- [ ] Unit tests :sob:
- [x] Refactor common code to `homie-core`
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
- [ ] Scripts
    - [ ] Local Deployment
    - [ ] CI

## Additional Features
- [ ] Expand dataset scope outside of Orange County
