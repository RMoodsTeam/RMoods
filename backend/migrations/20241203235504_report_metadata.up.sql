CREATE TABLE report_metadata (
    id                uuid PRIMARY KEY   DEFAULT uuid_generate_v4(),
    --
    report_created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    report_updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    --
    created_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE TRIGGER update_report_metadata_updated_at
    BEFORE UPDATE
    ON report_metadata
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();