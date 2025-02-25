-- Update version
UPDATE _info SET version = 3;

-- Add user images
ALTER TABLE `users` ADD `image_url` VARCHAR(50) NULL DEFAULT NULL AFTER `weight`;