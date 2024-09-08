CREATE TABLE jobs(
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL,
  status VARCHAR NOT NULL,
  create_by VARCHAR NOT NULL ,
  create_date TIMESTAMP  DEFAULT now()
)