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