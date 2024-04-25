CREATE TABLE tyields (
    term VARCHAR(20) NOT NULL,
    date DATE NOT NULL,
    yield_return FLOAT4,
    PRIMARY KEY (date, term)
);

