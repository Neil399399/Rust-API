-- Your SQL goes here

CREATE TABLE example_table (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR NOT NULL,
  firstname VARCHAR NOT NULL,
  lastname VARCHAR NOT NULL
);


INSERT INTO "example_table" ("id","email", "firstname","lastname")
VALUES ('41379e1f-5979-448e-b693-6cf5102e1cb4','neil@aetheras.io','neil','neil');