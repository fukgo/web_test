
create database test if not exists;
use test;


create table teacher(
    id int primary key auto_increment,
    name varchar(255) not null,
    uuid varchar(255) not null,
    picture_url varchar(255) not null,
    profile text
);
