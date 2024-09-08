-- Your SQL goes here
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    job_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    create_by VARCHAR NOT NULL,
    create_date TIMESTAMP  DEFAULT now()
)
