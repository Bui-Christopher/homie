CREATE TABLE tyields (
    yield_type VARCHAR(17) NOT NULL,
    date DATE NOT NULL,
    yield_return FLOAT NOT NULL,
    PRIMARY KEY (yield_type, date)
);
