-- Your SQL goes here
CREATE TABLE polls (
    question VARCHAR(255) NOT NULL,
    options VARCHAR(255) NOT NULL,
    author VARCHAR(11) NOT NULL,
    used BOOLEAN NOT NULL,
    add_options BOOLEAN NOT NULL,
    multiselect BOOLEAN NOT NULL,
    PRIMARY KEY (question)
);
CREATE TABLE poll_options (
    id INT NOT NULL AUTO_INCREMENT,
    question VARCHAR(255) NOT NULL,
    option_name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);
