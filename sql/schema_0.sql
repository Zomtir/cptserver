CREATE TABLE `_info` (
  `version` int(11) NOT NULL
);
CREATE TABLE `club_stocks` (
  `stock_id` int(11) NOT NULL AUTO_INCREMENT,
  `club_id` tinyint(4) NOT NULL,
  `item_id` int(11) NOT NULL,
  `storage` varchar(30) NOT NULL,
  `owned` int(11) NOT NULL,
  `loaned` int(11) NOT NULL,
  PRIMARY KEY (`stock_id`),
  KEY `REF_item` (`item_id`),
  KEY `REF_club` (`club_id`) USING BTREE,
  CONSTRAINT `club_stocks_ibfk_1` FOREIGN KEY (`club_id`) REFERENCES `clubs` (`club_id`) ON UPDATE CASCADE,
  CONSTRAINT `club_stocks_ibfk_2` FOREIGN KEY (`item_id`) REFERENCES `items` (`item_id`) ON UPDATE CASCADE
);
CREATE TABLE `clubs` (
  `club_id` tinyint(4) NOT NULL AUTO_INCREMENT,
  `club_key` varchar(10) NOT NULL,
  `name` varchar(30) NOT NULL,
  `description` varchar(100) NOT NULL,
  PRIMARY KEY (`club_id`),
  UNIQUE KEY `KEY` (`club_key`)
);
CREATE TABLE `course_bookmarks` (
  `course_id` mediumint(9) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`course_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `course_bookmarks_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  CONSTRAINT `course_bookmarks_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE
);
CREATE TABLE `course_leader_sieves` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1,
  PRIMARY KEY (`course_id`,`team_id`),
  KEY `REF_team` (`team_id`),
  CONSTRAINT `course_leader_sieves_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  CONSTRAINT `course_leader_sieves_ibfk_2` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE
);
CREATE TABLE `course_moderators` (
  `course_id` mediumint(9) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`course_id`,`user_id`),
  KEY `user_id` (`user_id`),
  CONSTRAINT `course_moderators_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  CONSTRAINT `course_moderators_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE
);
CREATE TABLE `course_participant_sieves` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1,
  PRIMARY KEY (`course_id`,`team_id`),
  KEY `REF_team` (`team_id`),
  CONSTRAINT `course_participant_sieves_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  CONSTRAINT `course_participant_sieves_ibfk_2` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE
);
CREATE TABLE `course_requirements` (
  `requirement_id` int(11) NOT NULL AUTO_INCREMENT,
  `course_id` mediumint(9) NOT NULL,
  `skill_id` smallint(6) NOT NULL,
  `rank` tinyint(4) NOT NULL,
  PRIMARY KEY (`requirement_id`),
  KEY `REF_skill` (`skill_id`),
  KEY `REF_course` (`course_id`),
  CONSTRAINT `course_requirements_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  CONSTRAINT `course_requirements_ibfk_2` FOREIGN KEY (`skill_id`) REFERENCES `skills` (`skill_id`) ON UPDATE CASCADE
);
CREATE TABLE `course_supporter_sieves` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1,
  PRIMARY KEY (`course_id`,`team_id`),
  KEY `REF_team` (`team_id`),
  CONSTRAINT `course_supporter_sieves_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  CONSTRAINT `course_supporter_sieves_ibfk_2` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE
);
CREATE TABLE `courses` (
  `course_id` mediumint(9) NOT NULL AUTO_INCREMENT,
  `course_key` char(10) NOT NULL,
  `title` varchar(100) NOT NULL,
  `active` tinyint(1) NOT NULL DEFAULT 1,
  `public` tinyint(1) NOT NULL DEFAULT 1,
  PRIMARY KEY (`course_id`),
  UNIQUE KEY `KEY` (`course_key`)
);
CREATE TABLE `event_bookmarks` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_bookmarks_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_bookmarks_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_leader_filters` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_leader_filters_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_leader_filters_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_leader_presences` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_leader_presences_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_leader_presences_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_leader_registrations` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `status` enum('POSITIVE','NEUTRAL','NEGATIVE','') NOT NULL,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_leader_registrations_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_leader_registrations_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_owners` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_owners_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_owners_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_participant_filters` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_participant_filters_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_participant_filters_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_participant_presences` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_participant_presences_ibfk_2` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_participant_presences_ibfk_3` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_participant_registrations` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `status` enum('POSITIVE','NEUTRAL','NEGATIVE','') NOT NULL,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_participant_registrations_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_participant_registrations_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_supporter_filters` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_supporter_filters_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_supporter_filters_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_supporter_presences` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_supporter_presences_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_supporter_presences_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `event_supporter_registrations` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `status` enum('POSITIVE','NEUTRAL','NEGATIVE','') NOT NULL,
  PRIMARY KEY (`event_id`,`user_id`),
  KEY `REF_user` (`user_id`),
  CONSTRAINT `event_supporter_registrations_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `event_supporter_registrations_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE `events` (
  `event_id` int(11) NOT NULL AUTO_INCREMENT,
  `event_key` char(12) NOT NULL,
  `pwd` tinytext NOT NULL,
  `title` varchar(100) NOT NULL,
  `begin` datetime NOT NULL,
  `end` datetime NOT NULL,
  `location_id` smallint(6) NOT NULL,
  `occurrence` enum('OCCURRING','CANCELED','VOIDED') NOT NULL DEFAULT 'OCCURRING',
  `acceptance` enum('DRAFT','PENDING','ACCEPTED','REJECTED') NOT NULL DEFAULT 'DRAFT',
  `public` tinyint(1) NOT NULL DEFAULT 0,
  `scrutable` tinyint(1) NOT NULL DEFAULT 1,
  `note` text NOT NULL DEFAULT '',
  `course_id` mediumint(9) DEFAULT NULL,
  PRIMARY KEY (`event_id`),
  UNIQUE KEY `KEY` (`event_key`),
  KEY `REF_course` (`course_id`),
  KEY `REF_location` (`location_id`),
  CONSTRAINT `events_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  CONSTRAINT `events_ibfk_2` FOREIGN KEY (`location_id`) REFERENCES `locations` (`location_id`) ON UPDATE CASCADE
);
CREATE TABLE `item_categories` (
  `category_id` smallint(6) NOT NULL AUTO_INCREMENT,
  `name` varchar(30) NOT NULL,
  PRIMARY KEY (`category_id`)
);
CREATE TABLE `items` (
  `item_id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(30) NOT NULL,
  `category_id` smallint(6) DEFAULT NULL,
  PRIMARY KEY (`item_id`),
  KEY `REF_CATEGORY` (`category_id`),
  CONSTRAINT `items_ibfk_1` FOREIGN KEY (`category_id`) REFERENCES `item_categories` (`category_id`) ON UPDATE CASCADE
);
CREATE TABLE `locations` (
  `location_id` smallint(6) NOT NULL AUTO_INCREMENT,
  `location_key` char(10) NOT NULL,
  `name` varchar(100) NOT NULL,
  `description` varchar(100) NOT NULL,
  PRIMARY KEY (`location_id`),
  UNIQUE KEY `KEY` (`location_key`)
);
CREATE TABLE `organisation_affiliations` (
  `organisation_id` smallint(6) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `member_identifier` char(10) DEFAULT NULL,
  `permission_solo_date` date DEFAULT NULL,
  `permission_team_date` date DEFAULT NULL,
  `residency_move_date` date DEFAULT NULL,
  KEY `organisation_members_ibfk_1` (`organisation_id`),
  KEY `organisation_members_ibfk_2` (`user_id`),
  CONSTRAINT `organisation_affiliations_ibfk_1` FOREIGN KEY (`organisation_id`) REFERENCES `organisations` (`organisation_id`) ON UPDATE CASCADE,
  CONSTRAINT `organisation_affiliations_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE
);
CREATE TABLE `organisations` (
  `organisation_id` smallint(6) NOT NULL AUTO_INCREMENT,
  `abbreviation` varchar(10) NOT NULL,
  `name` varchar(30) NOT NULL,
  PRIMARY KEY (`organisation_id`)
);
CREATE TABLE `skills` (
  `skill_id` smallint(6) NOT NULL AUTO_INCREMENT,
  `skill_key` char(10) NOT NULL,
  `title` tinytext NOT NULL,
  `min` tinyint(4) NOT NULL DEFAULT 0,
  `max` tinyint(4) NOT NULL DEFAULT 1,
  PRIMARY KEY (`skill_id`),
  UNIQUE KEY `KEY` (`skill_key`)
);
CREATE TABLE `team_members` (
  `team_id` mediumint(9) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`user_id`,`team_id`),
  KEY `REF_team` (`team_id`),
  CONSTRAINT `team_members_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE,
  CONSTRAINT `team_members_ibfk_3` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE
);
CREATE TABLE `teams` (
  `team_id` mediumint(9) NOT NULL AUTO_INCREMENT,
  `team_key` varchar(10) NOT NULL,
  `name` varchar(30) NOT NULL,
  `description` varchar(100) NOT NULL,
  `right_club_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_club_read` tinyint(1) NOT NULL DEFAULT 0,
  `right_competence_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_competence_read` tinyint(1) NOT NULL DEFAULT 0,
  `right_course_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_course_read` tinyint(1) NOT NULL DEFAULT 0,
  `right_event_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_event_read` tinyint(1) NOT NULL DEFAULT 0,
  `right_inventory_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_inventory_read` tinyint(1) NOT NULL DEFAULT 0,
  `right_location_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_location_read` tinyint(1) NOT NULL DEFAULT 0,
  `right_organisation_write` tinyint(1) NOT NULL,
  `right_organisation_read` tinyint(1) NOT NULL,
  `right_team_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_team_read` tinyint(1) NOT NULL DEFAULT 0,
  `right_user_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_user_read` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`team_id`),
  UNIQUE KEY `KEY` (`team_key`)
);
CREATE TABLE `terms` (
  `term_id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` mediumint(9) NOT NULL,
  `club_id` tinyint(4) NOT NULL,
  `term_begin` date DEFAULT NULL,
  `term_end` date DEFAULT NULL,
  PRIMARY KEY (`term_id`),
  KEY `user_id` (`user_id`),
  KEY `club_id` (`club_id`),
  CONSTRAINT `terms_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE,
  CONSTRAINT `terms_ibfk_2` FOREIGN KEY (`club_id`) REFERENCES `clubs` (`club_id`) ON UPDATE CASCADE
);
CREATE TABLE `user_competences` (
  `competence_id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` mediumint(9) NOT NULL,
  `skill_id` smallint(6) NOT NULL,
  `rank` tinyint(4) NOT NULL,
  `date` date NOT NULL DEFAULT '1000-01-01',
  `judge_id` mediumint(9) NOT NULL,
  PRIMARY KEY (`competence_id`),
  KEY `REF_judge` (`judge_id`),
  KEY `REF_user` (`user_id`) USING BTREE,
  KEY `REF_skill` (`skill_id`),
  CONSTRAINT `user_competences_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE,
  CONSTRAINT `user_competences_ibfk_2` FOREIGN KEY (`skill_id`) REFERENCES `skills` (`skill_id`) ON UPDATE CASCADE,
  CONSTRAINT `user_competences_ibfk_3` FOREIGN KEY (`judge_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE
);
CREATE TABLE `user_possessions` (
  `possession_id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` mediumint(9) NOT NULL,
  `item_id` int(11) NOT NULL,
  `acquisition_date` date NOT NULL,
  `owned` tinyint(1) NOT NULL DEFAULT 1,
  `stock_id` int(11) DEFAULT NULL,
  PRIMARY KEY (`possession_id`),
  KEY `REF_user` (`user_id`),
  KEY `REF_item` (`item_id`),
  KEY `REF_stock` (`stock_id`),
  CONSTRAINT `user_possessions_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE,
  CONSTRAINT `user_possessions_ibfk_2` FOREIGN KEY (`item_id`) REFERENCES `items` (`item_id`) ON UPDATE CASCADE,
  CONSTRAINT `user_possessions_ibfk_3` FOREIGN KEY (`stock_id`) REFERENCES `club_stocks` (`stock_id`) ON UPDATE CASCADE
);
CREATE TABLE `users` (
  `user_id` mediumint(9) NOT NULL AUTO_INCREMENT,
  `user_key` char(20) NOT NULL,
  `pwd` binary(32) NOT NULL,
  `pepper` binary(16) NOT NULL,
  `salt` binary(16) NOT NULL,
  `enabled` tinyint(1) NOT NULL DEFAULT 0,
  `active` tinyint(1) NOT NULL DEFAULT 1,
  `firstname` varchar(20) NOT NULL,
  `lastname` varchar(20) NOT NULL,
  `nickname` varchar(20) DEFAULT NULL,
  `address` varchar(60) DEFAULT NULL,
  `email` varchar(40) DEFAULT NULL,
  `phone` varchar(20) DEFAULT NULL,
  `iban` char(22) DEFAULT NULL,
  `birth_date` date DEFAULT NULL,
  `birth_location` varchar(60) DEFAULT NULL,
  `nationality` varchar(40) DEFAULT NULL,
  `gender` enum('MALE','FEMALE','OTHER') DEFAULT NULL,
  `height` smallint(6) DEFAULT NULL,
  `weight` smallint(6) DEFAULT NULL,
  `note` text DEFAULT NULL,
  PRIMARY KEY (`user_id`),
  UNIQUE KEY `KEY` (`user_key`)
);

