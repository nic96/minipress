create table if not exists posts
(
    id              uuid        primary key default uuid_generate_v4(),
    user_id         uuid        not null,
    title           text        not null constraint title_length check ( char_length(title) <= 255 ),
    slug            text        not null constraint slug_length check ( char_length(slug) <= 255 ),
    excerpt         text        not null,
    content         text        not null,
    created_at      timestamp   not null default now(),
    updated_at      timestamp   not null default now(),
    foreign key (user_id) references users(id)
);
create index on posts(slug);
create index on posts(title);
create index on posts(created_at);
