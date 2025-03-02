-- Update version
UPDATE _info SET version = 3;

-- Add user images
ALTER TABLE `users` ADD `image_url` VARCHAR(50) NULL DEFAULT NULL AFTER `weight`;

-- Add club banners
ALTER TABLE `clubs` ADD `banner_url` VARCHAR(50) NULL DEFAULT NULL AFTER `image_url`;
UPDATE `clubs` SET `banner_url` = image_url, `image_url` = NULL;

-- Add course attendance sieve roles
ALTER TABLE `course_participant_sieves` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'PARTICIPANT' AFTER `team_id`;
ALTER TABLE `course_leader_sieves` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'LEADER' AFTER `team_id`;
ALTER TABLE `course_supporter_sieves` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'SUPPORTER' AFTER `team_id`;

CREATE TABLE `course_attendance_sieves` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL,
  `role`  ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL,
  `access` tinyint(1) NOT NULL
);

ALTER TABLE `course_attendance_sieves`
ADD PRIMARY KEY (`course_id`,`team_id`,`role`),
ADD KEY `REF_team` (`team_id`);

ALTER TABLE `course_attendance_sieves`
ADD CONSTRAINT `course_attendance_sieves_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
ADD CONSTRAINT `course_attendance_sieves_ibfk_2` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE;

INSERT INTO course_attendance_sieves (`course_id`,`team_id`,`role`, `access`) SELECT `course_id`,`team_id`,`role`, `access` FROM course_participant_sieves;
INSERT INTO course_attendance_sieves (`course_id`,`team_id`,`role`, `access`) SELECT `course_id`,`team_id`,`role`, `access` FROM course_leader_sieves;
INSERT INTO course_attendance_sieves (`course_id`,`team_id`,`role`, `access`) SELECT `course_id`,`team_id`,`role`, `access` FROM course_supporter_sieves;

DROP TABLE `course_participant_sieves`, `course_leader_sieves`, `course_supporter_sieves`;

-- Add event attendance filter roles
ALTER TABLE `event_participant_filters` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'PARTICIPANT' AFTER `user_id`;
ALTER TABLE `event_leader_filters` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'LEADER' AFTER `user_id`;
ALTER TABLE `event_supporter_filters` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'SUPPORTER' AFTER `user_id`;

CREATE TABLE `event_attendance_filters` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `role`  ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL,
  `access` tinyint(1) NOT NULL
);

ALTER TABLE `event_attendance_filters`
ADD PRIMARY KEY (`event_id`,`user_id`,`role`),
ADD KEY `REF_user` (`user_id`);

ALTER TABLE `event_attendance_filters`
ADD CONSTRAINT `event_attendance_filters_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON UPDATE CASCADE,
ADD CONSTRAINT `event_attendance_filters_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

INSERT INTO event_attendance_filters (`event_id`,`user_id`,`role`, `access`) SELECT `event_id`,`user_id`,`role`, `access` FROM event_participant_filters;
INSERT INTO event_attendance_filters (`event_id`,`user_id`,`role`, `access`) SELECT `event_id`,`user_id`,`role`, `access` FROM event_leader_filters;
INSERT INTO event_attendance_filters (`event_id`,`user_id`,`role`, `access`) SELECT `event_id`,`user_id`,`role`, `access` FROM event_supporter_filters;

DROP TABLE `event_participant_filters`, `event_leader_filters`, `event_supporter_filters`;

-- Add event attendance presence roles
ALTER TABLE `event_participant_presences` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'PARTICIPANT' AFTER `user_id`;
ALTER TABLE `event_leader_presences` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'LEADER' AFTER `user_id`;
ALTER TABLE `event_supporter_presences` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'SUPPORTER' AFTER `user_id`;

CREATE TABLE `event_attendance_presences` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `role`  ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL
);

ALTER TABLE `event_attendance_presences`
ADD PRIMARY KEY (`event_id`,`user_id`,`role`),
ADD KEY `REF_user` (`user_id`);

ALTER TABLE `event_attendance_presences`
ADD CONSTRAINT `event_attendance_presences_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON UPDATE CASCADE,
ADD CONSTRAINT `event_attendance_presences_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

INSERT INTO event_attendance_presences (`event_id`,`user_id`,`role`) SELECT `event_id`,`user_id`,`role` FROM event_participant_presences;
INSERT INTO event_attendance_presences (`event_id`,`user_id`,`role`) SELECT `event_id`,`user_id`,`role` FROM event_leader_presences;
INSERT INTO event_attendance_presences (`event_id`,`user_id`,`role`) SELECT `event_id`,`user_id`,`role` FROM event_supporter_presences;

DROP TABLE `event_participant_presences`, `event_leader_presences`, `event_supporter_presences`;

-- Add event attendance registration roles
ALTER TABLE `event_participant_registrations` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'PARTICIPANT' AFTER `user_id`;
ALTER TABLE `event_leader_registrations` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'LEADER' AFTER `user_id`;
ALTER TABLE `event_supporter_registrations` ADD `role` ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL DEFAULT 'SUPPORTER' AFTER `user_id`;

CREATE TABLE `event_attendance_registrations` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `role`  ENUM('PARTICIPANT','LEADER','SUPPORTER','SPECTATOR') NOT NULL,
  `status` enum('POSITIVE','NEUTRAL','NEGATIVE','') NOT NULL
);

ALTER TABLE `event_attendance_registrations`
ADD PRIMARY KEY (`event_id`,`user_id`,`role`),
ADD KEY `REF_user` (`user_id`);

ALTER TABLE `event_attendance_registrations`
ADD CONSTRAINT `event_attendance_registrations_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON UPDATE CASCADE,
ADD CONSTRAINT `event_attendance_registrations_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

INSERT INTO event_attendance_registrations (`event_id`,`user_id`,`role`, `status`) SELECT `event_id`,`user_id`,`role`, `status` FROM event_participant_registrations;
INSERT INTO event_attendance_registrations (`event_id`,`user_id`,`role`, `status`) SELECT `event_id`,`user_id`,`role`, `status` FROM event_leader_registrations;
INSERT INTO event_attendance_registrations (`event_id`,`user_id`,`role`, `status`) SELECT `event_id`,`user_id`,`role`, `status` FROM event_supporter_registrations;

DROP TABLE `event_participant_registrations`, `event_leader_registrations`, `event_supporter_registrations`;