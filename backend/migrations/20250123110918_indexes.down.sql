-- NLP Analyses
DROP INDEX IF EXISTS idx_nlp_analyses_kind;
DROP INDEX IF EXISTS idx_nlp_analyses_report_id;

-- Data Sources table
DROP INDEX IF EXISTS idx_data_sources_name;
DROP INDEX IF EXISTS idx_data_sources_data_request_id;

-- Data Requests table
DROP INDEX IF EXISTS idx_data_requests_feed_kind;
DROP INDEX IF EXISTS idx_data_requests_report_id;

-- Reports table
DROP INDEX IF EXISTS idx_reports_state;
DROP INDEX IF EXISTS idx_reports_is_public;
DROP INDEX IF EXISTS idx_reports_user_id;

-- Users table
DROP INDEX IF EXISTS idx_users_name;
DROP INDEX IF EXISTS idx_users_email;
