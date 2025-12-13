-- Add migration script here

CREATE TABLE "workout_log" (
  "id" uuid PRIMARY KEY,
  "name" varchar(100) NOT NULL,
  "user_id" uuid,
  "workout_id" uuid,
  "start_at" timestamptz DEFAULT current_timestamp,
  "end_at" timestamptz,
  "deleted_at" timestamptz,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE,
  FOREIGN KEY ("workout_id") REFERENCES "workout" ("id") ON UPDATE CASCADE
);