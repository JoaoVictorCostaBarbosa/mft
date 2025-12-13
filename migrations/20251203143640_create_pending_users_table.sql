-- Add migration script here

CREATE TABLE "pending_users" (
  "id" uuid PRIMARY KEY,
  "name" varchar(100) NOT NULL,
  "email" varchar(255) UNIQUE NOT NULL,
  "password" text NOT NULL,
  "code" integer NOT NULL CHECK (code >= 100000 AND code <= 999999),
  "limit_date" timestamptz NOT NULL
);
