-- Add migration script here

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY,
  "name" varchar(100) NOT NULL,
  "email" varchar(255) UNIQUE NOT NULL,
  "password" text NOT NULL,
  "role" role_enum NOT NULL,
  "url_img" text,
  "created_at" timestamptz DEFAULT current_timestamp,
  "updated_at" timestamptz,
  "deleted_at" timestamptz
);