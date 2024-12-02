-- Update version
UPDATE _info SET version = 1;

-- Connect courses to clubs
ALTER TABLE `courses` ADD `club_id` TINYINT NULL DEFAULT NULL AFTER `public`;
ALTER TABLE `courses` ADD CONSTRAINT `courses_ibfk_1` FOREIGN KEY (`club_id`) REFERENCES `clubs`(`club_id`) ON DELETE RESTRICT ON UPDATE CASCADE;