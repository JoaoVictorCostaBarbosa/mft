-- Add migration script here

CREATE TABLE "refresh_token" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "token" text NOT NULL,
  "limit_date" timestamptz NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE ON DELETE CASCADE
);
