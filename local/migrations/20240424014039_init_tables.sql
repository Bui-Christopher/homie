CREATE TYPE term AS ENUM ('tenyear');
CREATE TYPE home_type AS ENUM ('allhomes', 'condococops', 'singlefamilyhomes');
CREATE TYPE region_type AS ENUM ('threezip', 'fivezip', 'city', 'county');
CREATE TYPE percentile AS ENUM ('bottom', 'middle', 'top');

CREATE TABLE tyields (
    term term NOT NULL,
    date DATE NOT NULL,
    yield_return FLOAT4,
    PRIMARY KEY (date, term)
);

CREATE TABLE hpis (
    region_type region_type NOT NULL,
    region_name VARCHAR(50) NOT NULL,
    year INTEGER NOT NULL,
    hpi FLOAT4,
    annual_change FLOAT4,
    hpi_1990_base FLOAT4,
    hpi_2000_base FLOAT4,
    PRIMARY KEY (region_name, year)
);

CREATE TABLE regions (
    city VARCHAR(50) NOT NULL,
    zipcode VARCHAR(10) NOT NULL,
    PRIMARY KEY (zipcode)
);

CREATE TABLE zhvi_metadata (
    home_type home_type NOT NULL,
    region_type region_type NOT NULL,
    region_name TEXT NOT NULL,
    percentile percentile NOT NULL,
    PRIMARY KEY (home_type, region_type, region_name, percentile)
);

CREATE TABLE zhvi_prices (
    home_type home_type NOT NULL,
    region_type region_type NOT NULL,
    region_name TEXT NOT NULL,
    percentile percentile NOT NULL,
    date DATE NOT NULL,
    value FLOAT8 NOT NULL,
    PRIMARY KEY (home_type, region_type, region_name, percentile, date),
    FOREIGN KEY (home_type, region_type, region_name, percentile) 
        REFERENCES zhvi_metadata(home_type, region_type, region_name, percentile)
);

