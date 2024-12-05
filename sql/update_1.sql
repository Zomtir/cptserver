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