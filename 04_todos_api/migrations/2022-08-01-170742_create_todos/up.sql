create extension if not exists "uuid-ossp";

create table if not exists todos (
  id uuid default uuid_generate_v4() primary key,
  title varchar(255) not null,
  checked boolean not null default false,
  created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
