DO
$$
    BEGIN
        CREATE TYPE nlp_analysis_kind AS ENUM (
            'clickbait',
            'hate_speech',
            'keywords',
            'language',
            'politics',
            'sarcasm',
            'sentiment',
            'spam'
            );
    EXCEPTION
        WHEN duplicate_object THEN NULL;
    END
$$;

CREATE TABLE IF NOT EXISTS nlp_analyses (
    id              uuid PRIMARY KEY           DEFAULT uuid_generate_v4(),
    nlp_metadata_id uuid              NOT NULL,
    kind            nlp_analysis_kind NOT NULL,
    analysis        jsonb             NOT NULL,
    --
    created_at      TIMESTAMP         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP         NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (nlp_metadata_id) REFERENCES nlp_metadata (id)
);

-- Create a trigger that calls the function before any update
CREATE OR REPLACE TRIGGER update_nlp_analyses_updated_at
    BEFORE UPDATE
    ON nlp_analyses
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
