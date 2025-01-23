-- Users table
CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_name ON users (name);

-- Reports table
CREATE INDEX idx_reports_user_id ON reports (user_id);
CREATE INDEX idx_reports_is_public ON reports (is_public);
-- Composite index for filtering reports by state
CREATE INDEX idx_reports_state ON reports (is_successful, is_error, is_in_progress);

-- Data Requests table
CREATE INDEX idx_data_requests_report_id ON data_requests (report_id);
CREATE INDEX idx_data_requests_feed_kind ON data_requests (feed_kind);

-- Data Sources table
CREATE INDEX idx_data_sources_data_request_id ON data_sources (data_request_id);
CREATE INDEX idx_data_sources_name ON data_sources (name);

-- NLP Analyses table
CREATE INDEX idx_nlp_analyses_report_id ON nlp_analyses (report_id);
CREATE INDEX idx_nlp_analyses_kind ON nlp_analyses (kind);
