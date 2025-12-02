-- Add migration script here

CREATE TABLE "workouts_in_plan" (
  "id" uuid PRIMARY KEY,
  "workout_plan_id" uuid NOT NULL,
  "workout_id" uuid NOT NULL,

  FOREIGN KEY ("workout_plan_id") REFERENCES "workout_plan" ("id") ON UPDATE CASCADE,
  FOREIGN KEY ("workout_id") REFERENCES "workout" ("id") ON UPDATE CASCADE
);