-- Update version
UPDATE _info SET version = 5;

-- Allow null values for number and expiration date in licenses
ALTER TABLE `licenses` CHANGE `number` `number` VARCHAR(20) NULL DEFAULT NULL;
ALTER TABLE `licenses` CHANGE `expiration` `expiration` DATE NULL DEFAULT NULL;
