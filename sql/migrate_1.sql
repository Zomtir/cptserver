-- Update version
UPDATE _info SET version = 1;

-- Connect courses to clubs
ALTER TABLE `courses` ADD `club_id` TINYINT NULL DEFAULT NULL AFTER `public`;
ALTER TABLE `courses` ADD CONSTRAINT `courses_ibfk_1` FOREIGN KEY (`club_id`) REFERENCES `clubs`(`club_id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- Add disciplines to clubs
ALTER TABLE `clubs` ADD `disciplines` VARCHAR(500) NULL DEFAULT NULL AFTER `description`;
-- Add image url to clubs
ALTER TABLE `clubs` ADD `image_url` VARCHAR(50) NULL DEFAULT NULL AFTER `disciplines`;
-- Add chairman to clubs
ALTER TABLE `clubs` ADD `chairman` VARCHAR(50) NULL DEFAULT NULL AFTER `image_url`;
-- Allow description to be NULL
ALTER TABLE `clubs` CHANGE `description` `description` VARCHAR(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL;

-- Add licenses
CREATE TABLE `licenses` (`id` MEDIUMINT NOT NULL AUTO_INCREMENT , `number` INT NOT NULL , `name` VARCHAR(50) NOT NULL , `expiration` DATE NOT NULL , PRIMARY KEY (`id`)) ENGINE = InnoDB;
-- Associate main and extra license with user
ALTER TABLE `users` ADD `license_main` MEDIUMINT NULL DEFAULT NULL AFTER `weight`, ADD `license_extra` MEDIUMINT NULL DEFAULT NULL AFTER `license_main`;
-- Add foreign key constraints
ALTER TABLE `users` ADD CONSTRAINT `users_ibfk_1` FOREIGN KEY (`license_main`) REFERENCES `licenses`(`id`) ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE `users` ADD CONSTRAINT `users_ibfk_2` FOREIGN KEY (`license_extra`) REFERENCES `licenses`(`id`) ON DELETE SET NULL ON UPDATE CASCADE;

-- Add bank accounts
CREATE TABLE `bank_accounts` (`id` MEDIUMINT NOT NULL AUTO_INCREMENT , `iban` CHAR(34) NOT NULL , `bic` CHAR(11) NOT NULL , `institute` VARCHAR(50) NOT NULL , PRIMARY KEY (`id`)) ENGINE = InnoDB;
-- Associate bank_account with user
ALTER TABLE `users` ADD `bank_account` MEDIUMINT NULL DEFAULT NULL AFTER `weight`;
-- Add foreign key constraints
ALTER TABLE `users` ADD CONSTRAINT `users_ibfk_3` FOREIGN KEY (`bank_account`) REFERENCES `bank_accounts`(`id`) ON DELETE SET NULL ON UPDATE CASCADE;

-- Transfer the IBAN

-- 1) Remove spaces from the IBAN
UPDATE users SET iban = REPLACE(iban, ' ', '') WHERE iban IS NOT NULL;
-- 2) Add a temporary user_id column to the bank_accounts table
ALTER TABLE bank_accounts ADD COLUMN temp_user_id MEDIUMINT NULL;
-- 3) Create a bank account for each user with an IBAN
INSERT INTO bank_accounts (iban, bic, institute, temp_user_id) SELECT iban, '', '', user_id FROM users WHERE iban IS NOT NULL;
-- 4) Update the user bank_accounts by using the temp_user_id
UPDATE users JOIN bank_accounts ON users.user_id = bank_accounts.temp_user_id SET users.bank_account = bank_accounts.id;
-- 6) Remove the temporary column
ALTER TABLE bank_accounts DROP COLUMN temp_user_id;
-- 7) Remove the IBAN column from users
ALTER TABLE users DROP COLUMN iban;
