-- Update version
UPDATE _info SET version = 4;

-- Fix organisation affiliations constraint
ALTER TABLE `organisation_affiliations` DROP INDEX `organisation_members_ibfk_1`, ADD PRIMARY KEY (`organisation_id`, `user_id`) USING BTREE;

-- Create disciplines table
CREATE TABLE `disciplines` (`discipline_id` SMALLINT NOT NULL AUTO_INCREMENT , `name` TINYTEXT NOT NULL , PRIMARY KEY (`discipline_id`));

-- Create term disciplines table
CREATE TABLE `term_disciplines` (`id` INT NOT NULL AUTO_INCREMENT , `term_id` INT NOT NULL , `discipline_id` SMALLINT NOT NULL , `begin` YEAR NULL , `end` YEAR NULL , PRIMARY KEY (`id`), KEY `REF_term` (`term_id`), KEY `REF_discipline` (`discipline_id`));
ALTER TABLE `term_disciplines` ADD CONSTRAINT `term_disciplines_ibfk_1` FOREIGN KEY (`term_id`) REFERENCES `terms`(`term_id`) ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE `term_disciplines` ADD CONSTRAINT `term_disciplines_ibfk_2` FOREIGN KEY (`discipline_id`) REFERENCES `disciplines`(`discipline_id`) ON DELETE RESTRICT ON UPDATE CASCADE;

--- Add discipline permissions
ALTER TABLE `teams` ADD `right_discipline_write` BOOLEAN NOT NULL DEFAULT FALSE AFTER `right_course_read`, ADD `right_discipline_read` BOOLEAN NOT NULL DEFAULT FALSE AFTER `right_discipline_write`;

--- Rename skill title to skill name
ALTER TABLE `skills` CHANGE `title` `name` TINYTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL;

--- Clean up redundant indexes
ALTER TABLE `organisation_affiliations` DROP INDEX `organisation_members_ibfk_2`;
ALTER TABLE `event_attendance_registrations` DROP INDEX `REF_user`;
ALTER TABLE `course_moderators` DROP INDEX `user_id`;
ALTER TABLE `course_bookmarks` DROP INDEX `REF_user`;
ALTER TABLE `course_attendance_sieves` DROP INDEX `REF_team`;
ALTER TABLE `event_owners` DROP INDEX `REF_user`;