CREATE TABLE IF NOT EXISTS report_analyses_maps (
    id             uuid PRIMARY KEY   DEFAULT uuid_generate_v4(),
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
    created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (clickbait_id) REFERENCES nlp_analyses (id),
    FOREIGN KEY (hate_speech_id) REFERENCES nlp_analyses (id),
    FOREIGN KEY (keywords_id) REFERENCES nlp_analyses (id),
    FOREIGN KEY (language_id) REFERENCES nlp_analyses (id),
    FOREIGN KEY (politics_id) REFERENCES nlp_analyses (id),
    FOREIGN KEY (sarcasm_id) REFERENCES nlp_analyses (id),
    FOREIGN KEY (sentiment_id) REFERENCES nlp_analyses (id),
    FOREIGN KEY (spam_id) REFERENCES nlp_analyses (id)
);
