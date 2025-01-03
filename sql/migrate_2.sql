-- Update version
UPDATE _info SET version = 2;

-- Increase nickname length
ALTER TABLE `users` CHANGE `nickname` `nickname` VARCHAR(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL DEFAULT NULL;

-- Add missing default values to organisation rights
ALTER TABLE `teams` CHANGE `right_organisation_read` `right_organisation_read` TINYINT(1) NOT NULL DEFAULT '0';
ALTER TABLE `teams` CHANGE `right_organisation_write` `right_organisation_write` TINYINT(1) NOT NULL DEFAULT '0';

-- Change license number from INT to VARCHAR
ALTER TABLE `licenses` CHANGE `number` `number` VARCHAR(20) NOT NULL;
