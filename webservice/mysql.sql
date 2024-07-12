use test;

create table course(
    id int primary key auto_increment,
    teacher_id int not null,
    name varchar(255) not null,
    description text not null,
    -- time字段默认为当前时间
    time datetime not null DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO course (teacher_id, name, description, time) 
VALUES 
(1, 'math', 'math course', '2020-01-01 00:00:00'),
(2, 'english', 'english course', '2020-01-02 00:00:00'),
(3, 'physics', 'physics course', '2020-01-03 00:00:00');