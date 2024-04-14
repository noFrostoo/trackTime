-- Add migration script here
create table if not exists issue
(
    id               text primary key not null,
    name             text unique not null,
    url              text        not null,
    summary          text        not null,
    assignee_email   text        not null,
    time_tracked_all text     not null
);

create table if not exists worklog
(
    id           text      primary key not null,
    issue_id     text      not null,
    start        text   not null,
    end          text   not null,
    total_time   text   not null,
    foreign key(issue_id) references issue(id)
);