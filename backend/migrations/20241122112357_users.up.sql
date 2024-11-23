CREATE TABLE users (
    id             VARCHAR(255) PRIMARY KEY,
    name           VARCHAR(255) NOT NULL,
    given_name     VARCHAR(255) NOT NULL,
    family_name    VARCHAR(255),
    picture        VARCHAR(255) NOT NULL,
    email          VARCHAR(255) NOT NULL,
    email_verified BOOLEAN      NOT NULL
);
