-- Your SQL goes here
create table random_matches (
    id serial primary key not null,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_id UUID not null references auth_users (id) on delete cascade,
    matched_user_id UUID  references auth_users (id) on delete set null
)