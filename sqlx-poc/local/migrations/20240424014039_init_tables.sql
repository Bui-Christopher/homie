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
