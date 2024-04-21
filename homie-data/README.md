# homie-data

## Description
This service is responsible for gathering datasets for the rest of the application.

It's in process of getting approval for the Zillow [API](https://www.bridgeinteractive.com/developers/zillow-group-data/).
In the meantime, it's data has been manually downloaded through Zillow (public datasetes)[https://www.zillow.com/research/data/].

## Bugs
- [ ] Incorrect parse on Zillow CSV's
    - [ ] Region

## TODO
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
    - [x] Split between postgres and in memory
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
