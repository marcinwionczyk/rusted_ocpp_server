-- Your SQL goes here
create table available_chargers
(
    serial_id  varchar(128) not null primary key,
    ip_address varchar(16)  not null
);

