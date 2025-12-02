-- Add migration script here

CREATE TABLE "measurements" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "weight" decimal(5,2),
  "height" decimal(5,2),
  "left_calf" decimal(5,1),
  "right_calf" decimal(5,1),
  "left_quadriceps" decimal(5,1),
  "right_quadriceps" decimal(5,1),
  "hip" decimal(5,1),
  "waist" decimal(5,1),
  "chest" decimal(5,1),
  "shoulders" decimal(5,1),
  "left_arm" decimal(5,1),
  "right_arm" decimal(5,1),
  "left_forearm" decimal(5,1),
  "right_forearm" decimal(5,1),
  "created_at" timestamp DEFAULT current_timestamp,
  "deleted_at" timestamp,

  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE
);