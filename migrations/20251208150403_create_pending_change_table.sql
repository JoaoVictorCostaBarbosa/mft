-- Add migration script here

CREATE TABLE "pending_change" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "code" integer NOT NULL CHECK (code >= 100000 AND code <= 999999),
  "limit_date" timestamptz NOT NULL,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE
);
