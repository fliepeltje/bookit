create table contractor
(
    slug varchar(15) not null primary key,
    name varchar(255) not null
);

create table alias
(
    slug varchar(15) not null primary key,
    contractor varchar(15) not null,
    rate integer not null,
    foreign key(contractor) references contractor(slug)
);

create table timelog
(
    hash varchar(15) not null primary key,
    alias varchar(15) not null,
    minutes integer not null,
    date date not null,
    message varchar(255) null,
    ticket varchar(15) null,
    timestamp datetime not null,
    foreign key(alias) references alias(slug)
);