# homie-data

## Description
This service is responsible for gathering datasets for the rest of the application.

It's in process of getting approval for the Zillow [API](https://www.bridgeinteractive.com/developers/zillow-group-data/).
In the meantime, it's data has been manually downloaded through Zillow (public datasetes)[https://www.zillow.com/research/data/].

## TODO
- [ ] Read remaining datasets
    - [ ] AllHomes
        - [ ] Bottom-tier
            - [ ] City
            - [ ] County
        - [ ] Top-tier
            - [ ] City
            - [ ] County
    - [ ] CondoCoops
        - [ ] City
        - [ ] County
        - [ ] Zip
    - [ ] SingleFamilyHomes
        - [ ] City
        - [ ] County
        - [ ] Zip
- [ ] Write datasets to DB (postgreSQL)
    - [ ] fed_h15
    - [ ] fhfa-hpi
    - [ ] cities/counties/zipcodes
    - [ ] zillow-zhvi
- [ ] Unit tests :sob:
- [ ] Refactor common code to `homie-core`
- [ ] Double check what structs/mods are exposed may limit it to the crate
