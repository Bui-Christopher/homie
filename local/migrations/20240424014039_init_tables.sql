CREATE TABLE tyields (
    id SERIAL PRIMARY KEY,
    yield_type VARCHAR(20) NOT NULL,
    date DATE NOT NULL,
    yield_return FLOAT NOT NULL
);
