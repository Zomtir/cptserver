-- Update version
UPDATE _info SET version = 3;

-- Add user images
ALTER TABLE `users` ADD `image_url` VARCHAR(50) NULL DEFAULT NULL AFTER `weight`;

-- Add club banners
ALTER TABLE `clubs` ADD `banner_url` VARCHAR(50) NULL DEFAULT NULL AFTER `image_url`;
UPDATE `clubs` SET `banner_url` = image_url, `image_url` = NULL;
