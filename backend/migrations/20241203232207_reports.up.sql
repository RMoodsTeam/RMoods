CREATE TABLE IF NOT EXISTS reports (
    id              uuid PRIMARY KEY      DEFAULT uuid_generate_v4(),
    --
    display_id      VARCHAR(32)  NOT NULL UNIQUE,
    user_id         VARCHAR(255) NOT NULL,
    title           VARCHAR(255) NOT NULL,
    description     TEXT         NOT NULL,
    is_public       BOOLEAN      NOT NULL,
    metadata_id     uuid         NOT NULL UNIQUE,
    analyses_map_id uuid         NOT NULL UNIQUE,
    --
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (metadata_id) REFERENCES nlp_metadata (id),
    FOREIGN KEY (analyses_map_id) REFERENCES report_analyses_maps (id)
);

-- Create a trigger that calls the function before any update
CREATE OR REPLACE TRIGGER update_reports_updated_at
    BEFORE UPDATE
    ON reports
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
