create table if not exists users
(
    id                  uuid        primary key default uuid_generate_v4(),
    username            text        not null unique constraint username_length check (char_length(username) <= 39 ),
    email               text        null constraint email_length check ( char_length(email) <= 255 ),
    password            text        null,
    name                text        null constraint name_length check ( char_length(name) <= 255 ),
    avatar_url          text        null constraint avatar_url_length check ( char_length(avatar_url) <= 255 ),
    gravatar_id         text        null constraint gravatar_id_length check ( char_length(gravatar_id) <= 255 ),
    github_id           bigint      null unique,
    github_token        text        null constraint github_token_length check ( char_length(github_token) <= 255 ),
    role                smallint    not null default 5,
    created_at          timestamp   not null default now(),
    updated_at          timestamp   not null default now()
);
create index on users(created_at);
comment on column users.role is '1 - Super Admin, 2 - Admin, 3 - Editor, 4 - Author, 5 - Contributor, 5 - Subscriber';
