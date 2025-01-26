-- Enable uuid_generate_v4() function
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    google_id      TEXT PRIMARY KEY,
    --
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

CREATE TABLE IF NOT EXISTS reports (
    id             TEXT PRIMARY KEY,
    --
    user_id        TEXT        NOT NULL, -- TEXT because it's a Google ID, not a UUID
    title          TEXT        NOT NULL,
    description    TEXT        NOT NULL,
    is_public      BOOLEAN     NOT NULL,
    is_successful  BOOLEAN     NOT NULL,
    is_in_progress BOOLEAN     NOT NULL,
    is_error       BOOLEAN     NOT NULL,
    error_message  TEXT                 DEFAULT NULL,
    --
    created_at     timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT report_state_error_always_has_message
        CHECK
            ((is_error = TRUE AND error_message IS NOT NULL) OR
             (is_error = FALSE AND error_message IS NULL)),

    -- xor checks, only one of those state columns can be true
    CONSTRAINT report_state_xor
        CHECK ((is_successful AND NOT is_error AND NOT is_in_progress) OR
               (NOT is_successful AND is_error AND NOT is_in_progress) OR
               (NOT is_successful AND NOT is_error AND is_in_progress)),

    FOREIGN KEY (user_id) REFERENCES users (google_id) ON DELETE CASCADE
);

CREATE TABLE data_requests (
    id           uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    report_id    TEXT        NOT NULL UNIQUE,
    --
    feed_kind    TEXT        NOT NULL,
    size         INT         NOT NULL,
    sort_by_kind TEXT        NOT NULL,
    sort_by_time TEXT,
    --
    created_at   timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (report_id) REFERENCES reports (id) ON DELETE CASCADE
);

CREATE TABLE data_sources (
    id              uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    data_request_id uuid        NOT NULL,
    --
    name            TEXT        NOT NULL,
    post_id         TEXT,
    share           INT         NOT NULL,
    --
    created_at      timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (data_request_id) REFERENCES data_requests (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS nlp_analyses (
    id           uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    report_id    TEXT        NOT NULL, -- the map that this analysis belongs to
    --
    kind         TEXT        NOT NULL,
    generated_in FLOAT       NOT NULL,
    analysis     jsonb       NOT NULL,
    --
    created_at   timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (report_id) REFERENCES reports (id) ON DELETE CASCADE
);

-- Create a function to automatically update the updated_at column
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    new.updated_at = CURRENT_TIMESTAMP;
    RETURN new;
END;
$$ LANGUAGE 'plpgsql';

CREATE OR REPLACE TRIGGER update_reports_updated_at
    BEFORE UPDATE
    ON reports
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER update_nlp_analyses_updated_at
    BEFORE UPDATE
    ON nlp_analyses
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER update_users_updated_at
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER update_data_requests_updated_at
    BEFORE UPDATE
    ON data_requests
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE TRIGGER update_data_sources_updated_at
    BEFORE UPDATE
    ON data_sources
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();