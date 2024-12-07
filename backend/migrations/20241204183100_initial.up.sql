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

CREATE TABLE IF NOT EXISTS users (
    id             uuid PRIMARY KEY,
    --
    google_sub     TEXT        NOT NULL,
    name           TEXT        NOT NULL,
    given_name     TEXT        NOT NULL,
    family_name    TEXT,
    picture        TEXT        NOT NULL,
    email          TEXT        NOT NULL,
    email_verified BOOLEAN     NOT NULL,
    --
    created_at     timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE TRIGGER update_users_updated_at
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS nlp_metadata (
    id           uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    --
    generated_in FLOAT       NOT NULL,
    --
    created_at   timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE TRIGGER update_nlp_metadata_updated_at
    BEFORE UPDATE
    ON nlp_metadata
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS nlp_analyses (
    id              uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    --
    nlp_metadata_id uuid        NOT NULL,
    kind            TEXT        NOT NULL,
    analysis        jsonb       NOT NULL,
    --
    created_at      timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (nlp_metadata_id) REFERENCES nlp_metadata (id) ON DELETE CASCADE
);

CREATE OR REPLACE TRIGGER update_nlp_analyses_updated_at
    BEFORE UPDATE
    ON nlp_analyses
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE IF NOT EXISTS report_analyses_maps (
    id             uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    --
    clickbait_id   uuid,
    hate_speech_id uuid,
    keywords_id    uuid,
    language_id    uuid,
    politics_id    uuid,
    sarcasm_id     uuid,
    sentiment_id   uuid,
    spam_id        uuid,
    --
    created_at     timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (clickbait_id) REFERENCES nlp_analyses (id) ON DELETE CASCADE,
    FOREIGN KEY (hate_speech_id) REFERENCES nlp_analyses (id) ON DELETE CASCADE,
    FOREIGN KEY (keywords_id) REFERENCES nlp_analyses (id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES nlp_analyses (id) ON DELETE CASCADE,
    FOREIGN KEY (politics_id) REFERENCES nlp_analyses (id) ON DELETE CASCADE,
    FOREIGN KEY (sarcasm_id) REFERENCES nlp_analyses (id) ON DELETE CASCADE,
    FOREIGN KEY (sentiment_id) REFERENCES nlp_analyses (id) ON DELETE CASCADE,
    FOREIGN KEY (spam_id) REFERENCES nlp_analyses (id) ON DELETE CASCADE
);

CREATE OR REPLACE TRIGGER report_analyses_maps_updated_at
    BEFORE UPDATE
    ON report_analyses_maps
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE IF NOT EXISTS report_metadata (
    id                uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    --
    report_created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    report_updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    --
    created_at        timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE TRIGGER update_report_metadata_updated_at
    BEFORE UPDATE
    ON report_metadata
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE IF NOT EXISTS reports (
    id              uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    --
    display_id      TEXT        NOT NULL UNIQUE,
    user_id         uuid        NOT NULL,
    title           TEXT        NOT NULL,
    description     TEXT        NOT NULL,
    is_public       BOOLEAN     NOT NULL,
    metadata_id     uuid        NOT NULL UNIQUE,
    analyses_map_id uuid        NOT NULL UNIQUE,
    --
    created_at      timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users (id), -- DO NOT CASCADE
    FOREIGN KEY (metadata_id) REFERENCES nlp_metadata (id) ON DELETE CASCADE,
    FOREIGN KEY (analyses_map_id) REFERENCES report_analyses_maps (id) ON DELETE CASCADE
);

CREATE OR REPLACE TRIGGER update_reports_updated_at
    BEFORE UPDATE
    ON reports
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
