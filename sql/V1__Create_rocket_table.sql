DROP TABLE IF EXISTS rocket;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE rocket (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    launch_date DATE
);

INSERT INTO rocket (name, launch_date) VALUES
('Falcon 9', '2024-06-18'),
('Falcon 9', '2023-06-19'),
('Falcon Heavy', '2023-06-25'),
('Electron', '2023-06-26'),
('H-3', '2023-06-30');
