drop table if exists course;

create table course
(
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default now()
);

insert into course
    (id, teacher_id, name, time)
values(1,1,'First course','2024-07-04 01:06:00');
insert into course
    (id, teacher_id, name, time)
values(2,1,'Second course','2024-07-04 01:32:00');