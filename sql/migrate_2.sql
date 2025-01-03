-- Update version
UPDATE _info SET version = 2;

-- Increase nickname length
ALTER TABLE `users` CHANGE `nickname` `nickname` VARCHAR(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL DEFAULT NULL;
