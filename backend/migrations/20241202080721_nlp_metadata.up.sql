-- Create a function to automatically update the updated_at column
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    new.updated_at = CURRENT_TIMESTAMP;
    RETURN new;
END;
$$ LANGUAGE 'plpgsql';

-- Enable uuid_generate_v4() function
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS nlp_metadata (
    id           uuid PRIMARY KEY   DEFAULT uuid_generate_v4(),
    --
    generated_in FLOAT     NOT NULL,
    --
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create a trigger that calls the function before any update
CREATE OR REPLACE TRIGGER update_nlp_metadata_updated_at
    BEFORE UPDATE
    ON nlp_metadata
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();