-- Update version
UPDATE _info SET version = 4;

-- Fix organisation affiliations constraint
ALTER TABLE `organisation_affiliations` DROP INDEX `organisation_members_ibfk_1`, ADD PRIMARY KEY (`organisation_id`, `user_id`) USING BTREE;

-- Create disciplines table
CREATE TABLE `disciplines` (`discipline_id` SMALLINT NOT NULL AUTO_INCREMENT , `name` TINYTEXT NOT NULL , PRIMARY KEY (`discipline_id`));

-- Create term disciplines table
CREATE TABLE `term_disciplines` (`id` INT NOT NULL AUTO_INCREMENT , `term_id` INT NOT NULL , `discipline_id` SMALLINT NOT NULL , `begin` YEAR NULL , `end` YEAR NULL , PRIMARY KEY (`id`));
ALTER TABLE `term_disciplines` ADD CONSTRAINT `term_disciplines_ibfk_1` FOREIGN KEY (`term_id`) REFERENCES `terms`(`term_id`) ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE `term_disciplines` ADD CONSTRAINT `term_disciplines_ibfk_2` FOREIGN KEY (`discipline_id`) REFERENCES `disciplines`(`discipline_id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- Create table that contains items required for each discipline by a user
CREATE TABLE `user_discipline_items` (`id` INT NOT NULL AUTO_INCREMENT , `user_id` MEDIUMINT NOT NULL , `discipline_id` SMALLINT NOT NULL , `item_id` INT NOT NULL , `count` INT NOT NULL , PRIMARY KEY (`id`));
ALTER TABLE `user_discipline_items` ADD CONSTRAINT `user_discipline_items_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users`(`user_id`) ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE `user_discipline_items` ADD CONSTRAINT `user_discipline_items_ibfk_2` FOREIGN KEY (`discipline_id`) REFERENCES `disciplines`(`discipline_id`) ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE `user_discipline_items` ADD CONSTRAINT `user_discipline_items_ibfk_3` FOREIGN KEY (`item_id`) REFERENCES `items`(`item_id`) ON DELETE RESTRICT ON UPDATE CASCADE;
