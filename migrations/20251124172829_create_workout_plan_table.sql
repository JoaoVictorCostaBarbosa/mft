-- Add migration script here

CREATE TABLE "workout_plan" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "name" varchar(100) NOT NULL,
  "created_at" timestamp DEFAULT current_timestamp,
  "updated_at" timestamp,
  "deleted_at" timestamp,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE
);