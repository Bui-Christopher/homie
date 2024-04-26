CREATE TABLE tyields (
    term VARCHAR(20) NOT NULL,
    date DATE NOT NULL,
    yield_return FLOAT4,
    PRIMARY KEY (date, term)
);

CREATE TABLE hpis (
    region TEXT NOT NULL,
    year INTEGER NOT NULL,
    hpi FLOAT4,
    annual_change FLOAT4,
    hpi_1990_base FLOAT4,
    hpi_2000_base FLOAT4,
    PRIMARY KEY (region, year)
);

CREATE TABLE zhvi_metadata (
    home_type VARCHAR(20) NOT NULL,
    region_type VARCHAR(20) NOT NULL,
    region_name TEXT NOT NULL,
    percentile VARCHAR(20) NOT NULL,
    PRIMARY KEY (home_type, region_type, region_name, percentile)
);

CREATE TABLE zhvi_prices (
    home_type VARCHAR(20) NOT NULL,
    region_type VARCHAR(20) NOT NULL,
    region_name TEXT NOT NULL,
    percentile VARCHAR(20) NOT NULL,
    date DATE NOT NULL,
    value FLOAT8 NOT NULL,
    PRIMARY KEY (home_type, region_type, region_name, percentile),
    FOREIGN KEY (home_type, region_type, region_name, percentile) 
        REFERENCES zhvi_metadata(home_type, region_type, region_name, percentile)
);

