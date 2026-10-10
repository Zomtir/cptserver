-- Update version
UPDATE _info SET version = 5;

-- Allow null values for number and expiration date in licenses
ALTER TABLE `licenses` CHANGE `number` `number` VARCHAR(20) NULL DEFAULT NULL;
ALTER TABLE `licenses` CHANGE `expiration` `expiration` DATE NULL DEFAULT NULL;

-- Add club compensations
CREATE TABLE `club_compensations` (`club_compensation_id` INT NOT NULL AUTO_INCREMENT , `club_id` TINYINT NOT NULL , `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL , `compensation` DECIMAL(5, 2) NOT NULL , PRIMARY KEY (`club_compensation_id`));
ALTER TABLE `club_compensations` ADD INDEX `REF_club` (`club_id`);
ALTER TABLE `club_compensations` ADD CONSTRAINT `club_compensations_ibfk_1` FOREIGN KEY (`club_id`) REFERENCES `clubs`(`club_id`) ON UPDATE CASCADE;
