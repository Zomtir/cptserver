-- phpMyAdmin SQL Dump
-- version 5.2.1deb3
-- https://www.phpmyadmin.net/
--
-- Host: localhost:8001
-- Generation Time: Jul 04, 2024 at 06:11 AM
-- Server version: 10.11.8-MariaDB-0ubuntu0.24.04.1
-- PHP Version: 8.3.6

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `cptdb`
--

-- --------------------------------------------------------

--
-- Table structure for table `clubs`
--

CREATE TABLE `clubs` (
  `club_id` tinyint(4) NOT NULL,
  `club_key` varchar(10) NOT NULL,
  `name` varchar(30) NOT NULL,
  `description` varchar(100) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `club_stocks`
--

CREATE TABLE `club_stocks` (
  `club_id` tinyint(4) NOT NULL,
  `item_id` int(11) NOT NULL,
  `owned` int(11) NOT NULL,
  `loaned` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `courses`
--

CREATE TABLE `courses` (
  `course_id` mediumint(9) NOT NULL,
  `course_key` char(10) NOT NULL,
  `title` varchar(100) NOT NULL,
  `active` tinyint(1) NOT NULL DEFAULT 1,
  `public` tinyint(1) NOT NULL DEFAULT 1
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `course_bookmarks`
--

CREATE TABLE `course_bookmarks` (
  `course_id` mediumint(9) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `course_leader_sieves`
--

CREATE TABLE `course_leader_sieves` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `course_moderators`
--

CREATE TABLE `course_moderators` (
  `course_id` mediumint(9) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `course_participant_sieves`
--

CREATE TABLE `course_participant_sieves` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `course_requirements`
--

CREATE TABLE `course_requirements` (
  `requirement_id` int(11) NOT NULL,
  `course_id` mediumint(9) NOT NULL,
  `skill_id` smallint(6) NOT NULL,
  `rank` tinyint(4) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `course_supporter_sieves`
--

CREATE TABLE `course_supporter_sieves` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `events`
--

CREATE TABLE `events` (
  `event_id` int(11) NOT NULL,
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
  `course_id` mediumint(9) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_bookmarks`
--

CREATE TABLE `event_bookmarks` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_leader_filters`
--

CREATE TABLE `event_leader_filters` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_leader_presences`
--

CREATE TABLE `event_leader_presences` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_leader_registrations`
--

CREATE TABLE `event_leader_registrations` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `status` enum('POSITIVE','NEUTRAL','NEGATIVE','') NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_owners`
--

CREATE TABLE `event_owners` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_participant_filters`
--

CREATE TABLE `event_participant_filters` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_participant_presences`
--

CREATE TABLE `event_participant_presences` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_participant_registrations`
--

CREATE TABLE `event_participant_registrations` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `status` enum('POSITIVE','NEUTRAL','NEGATIVE','') NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_supporter_filters`
--

CREATE TABLE `event_supporter_filters` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `access` tinyint(1) NOT NULL DEFAULT 1
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_supporter_presences`
--

CREATE TABLE `event_supporter_presences` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `event_supporter_registrations`
--

CREATE TABLE `event_supporter_registrations` (
  `event_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `status` enum('POSITIVE','NEUTRAL','NEGATIVE','') NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `items`
--

CREATE TABLE `items` (
  `item_id` int(11) NOT NULL,
  `name` varchar(30) NOT NULL,
  `category_id` smallint(6) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `item_categories`
--

CREATE TABLE `item_categories` (
  `category_id` smallint(6) NOT NULL,
  `name` varchar(30) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `locations`
--

CREATE TABLE `locations` (
  `location_id` smallint(6) NOT NULL,
  `location_key` char(10) NOT NULL,
  `name` varchar(100) NOT NULL,
  `description` varchar(100) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `skills`
--

CREATE TABLE `skills` (
  `skill_id` smallint(6) NOT NULL,
  `skill_key` char(10) NOT NULL,
  `title` tinytext NOT NULL,
  `min` tinyint(4) NOT NULL DEFAULT 0,
  `max` tinyint(4) NOT NULL DEFAULT 1
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `teams`
--

CREATE TABLE `teams` (
  `team_id` mediumint(9) NOT NULL,
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
  `right_team_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_team_read` tinyint(1) NOT NULL DEFAULT 0,
  `right_user_write` tinyint(1) NOT NULL DEFAULT 0,
  `right_user_read` tinyint(1) NOT NULL DEFAULT 0
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `team_members`
--

CREATE TABLE `team_members` (
  `team_id` mediumint(9) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `terms`
--

CREATE TABLE `terms` (
  `term_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `club_id` tinyint(4) NOT NULL,
  `term_begin` date DEFAULT NULL,
  `term_end` date DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `users`
--

CREATE TABLE `users` (
  `user_id` mediumint(9) NOT NULL,
  `user_key` char(12) NOT NULL,
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
  `birthday` date DEFAULT NULL,
  `birthlocation` varchar(60) DEFAULT NULL,
  `nationality` varchar(40) DEFAULT NULL,
  `gender` enum('MALE','FEMALE','OTHER') DEFAULT NULL,
  `federationNumber` mediumint(9) DEFAULT NULL,
  `federationPermissionSolo` date DEFAULT NULL,
  `federationPermissionTeam` date DEFAULT NULL,
  `federationResidency` date DEFAULT NULL,
  `dataDeclaration` tinyint(1) DEFAULT NULL,
  `dataDisclaimer` text DEFAULT NULL,
  `note` text DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `user_competences`
--

CREATE TABLE `user_competences` (
  `competence_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `skill_id` smallint(6) NOT NULL,
  `rank` tinyint(4) NOT NULL,
  `date` date NOT NULL DEFAULT '1000-01-01',
  `judge_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `user_possessions`
--

CREATE TABLE `user_possessions` (
  `possession_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL,
  `item_id` int(11) NOT NULL,
  `owned` tinyint(1) NOT NULL DEFAULT 1,
  `club_id` tinyint(4) DEFAULT NULL,
  `transfer_date` date DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Indexes for dumped tables
--

--
-- Indexes for table `clubs`
--
ALTER TABLE `clubs`
  ADD PRIMARY KEY (`club_id`),
  ADD UNIQUE KEY `KEY` (`club_key`);

--
-- Indexes for table `club_stocks`
--
ALTER TABLE `club_stocks`
  ADD PRIMARY KEY (`club_id`,`item_id`),
  ADD KEY `REF_item` (`item_id`);

--
-- Indexes for table `courses`
--
ALTER TABLE `courses`
  ADD PRIMARY KEY (`course_id`),
  ADD UNIQUE KEY `KEY` (`course_key`);

--
-- Indexes for table `course_bookmarks`
--
ALTER TABLE `course_bookmarks`
  ADD PRIMARY KEY (`course_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `course_leader_sieves`
--
ALTER TABLE `course_leader_sieves`
  ADD PRIMARY KEY (`course_id`,`team_id`),
  ADD KEY `REF_team` (`team_id`);

--
-- Indexes for table `course_moderators`
--
ALTER TABLE `course_moderators`
  ADD PRIMARY KEY (`course_id`,`user_id`),
  ADD KEY `user_id` (`user_id`);

--
-- Indexes for table `course_participant_sieves`
--
ALTER TABLE `course_participant_sieves`
  ADD PRIMARY KEY (`course_id`,`team_id`),
  ADD KEY `REF_team` (`team_id`);

--
-- Indexes for table `course_requirements`
--
ALTER TABLE `course_requirements`
  ADD PRIMARY KEY (`requirement_id`),
  ADD KEY `REF_skill` (`skill_id`),
  ADD KEY `REF_course` (`course_id`);

--
-- Indexes for table `course_supporter_sieves`
--
ALTER TABLE `course_supporter_sieves`
  ADD PRIMARY KEY (`course_id`,`team_id`),
  ADD KEY `REF_team` (`team_id`);

--
-- Indexes for table `events`
--
ALTER TABLE `events`
  ADD PRIMARY KEY (`event_id`),
  ADD UNIQUE KEY `KEY` (`event_key`),
  ADD KEY `REF_course` (`course_id`),
  ADD KEY `REF_location` (`location_id`);

--
-- Indexes for table `event_bookmarks`
--
ALTER TABLE `event_bookmarks`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_leader_filters`
--
ALTER TABLE `event_leader_filters`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_leader_presences`
--
ALTER TABLE `event_leader_presences`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_leader_registrations`
--
ALTER TABLE `event_leader_registrations`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_owners`
--
ALTER TABLE `event_owners`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_participant_filters`
--
ALTER TABLE `event_participant_filters`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_participant_presences`
--
ALTER TABLE `event_participant_presences`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_participant_registrations`
--
ALTER TABLE `event_participant_registrations`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_supporter_filters`
--
ALTER TABLE `event_supporter_filters`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_supporter_presences`
--
ALTER TABLE `event_supporter_presences`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `event_supporter_registrations`
--
ALTER TABLE `event_supporter_registrations`
  ADD PRIMARY KEY (`event_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`);

--
-- Indexes for table `items`
--
ALTER TABLE `items`
  ADD PRIMARY KEY (`item_id`),
  ADD KEY `REF_CATEGORY` (`category_id`);

--
-- Indexes for table `item_categories`
--
ALTER TABLE `item_categories`
  ADD PRIMARY KEY (`category_id`);

--
-- Indexes for table `locations`
--
ALTER TABLE `locations`
  ADD PRIMARY KEY (`location_id`),
  ADD UNIQUE KEY `KEY` (`location_key`);

--
-- Indexes for table `skills`
--
ALTER TABLE `skills`
  ADD PRIMARY KEY (`skill_id`),
  ADD UNIQUE KEY `KEY` (`skill_key`);

--
-- Indexes for table `teams`
--
ALTER TABLE `teams`
  ADD PRIMARY KEY (`team_id`),
  ADD UNIQUE KEY `KEY` (`team_key`);

--
-- Indexes for table `team_members`
--
ALTER TABLE `team_members`
  ADD PRIMARY KEY (`user_id`,`team_id`),
  ADD KEY `REF_team` (`team_id`);

--
-- Indexes for table `terms`
--
ALTER TABLE `terms`
  ADD PRIMARY KEY (`term_id`),
  ADD KEY `user_id` (`user_id`),
  ADD KEY `club_id` (`club_id`);

--
-- Indexes for table `users`
--
ALTER TABLE `users`
  ADD PRIMARY KEY (`user_id`),
  ADD UNIQUE KEY `KEY` (`user_key`);

--
-- Indexes for table `user_competences`
--
ALTER TABLE `user_competences`
  ADD PRIMARY KEY (`competence_id`),
  ADD KEY `REF_judge` (`judge_id`),
  ADD KEY `REF_user` (`user_id`) USING BTREE,
  ADD KEY `REF_skill` (`skill_id`);

--
-- Indexes for table `user_possessions`
--
ALTER TABLE `user_possessions`
  ADD PRIMARY KEY (`possession_id`),
  ADD KEY `REF_user` (`user_id`),
  ADD KEY `REF_item` (`item_id`),
  ADD KEY `REF_club` (`club_id`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `clubs`
--
ALTER TABLE `clubs`
  MODIFY `club_id` tinyint(4) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `courses`
--
ALTER TABLE `courses`
  MODIFY `course_id` mediumint(9) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `course_requirements`
--
ALTER TABLE `course_requirements`
  MODIFY `requirement_id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `events`
--
ALTER TABLE `events`
  MODIFY `event_id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `items`
--
ALTER TABLE `items`
  MODIFY `item_id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `item_categories`
--
ALTER TABLE `item_categories`
  MODIFY `category_id` smallint(6) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `locations`
--
ALTER TABLE `locations`
  MODIFY `location_id` smallint(6) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `skills`
--
ALTER TABLE `skills`
  MODIFY `skill_id` smallint(6) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `teams`
--
ALTER TABLE `teams`
  MODIFY `team_id` mediumint(9) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `terms`
--
ALTER TABLE `terms`
  MODIFY `term_id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `users`
--
ALTER TABLE `users`
  MODIFY `user_id` mediumint(9) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `user_competences`
--
ALTER TABLE `user_competences`
  MODIFY `competence_id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `user_possessions`
--
ALTER TABLE `user_possessions`
  MODIFY `possession_id` int(11) NOT NULL AUTO_INCREMENT;

--
-- Constraints for dumped tables
--

--
-- Constraints for table `club_stocks`
--
ALTER TABLE `club_stocks`
  ADD CONSTRAINT `club_stocks_ibfk_1` FOREIGN KEY (`club_id`) REFERENCES `clubs` (`club_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `club_stocks_ibfk_2` FOREIGN KEY (`item_id`) REFERENCES `items` (`item_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_bookmarks`
--
ALTER TABLE `course_bookmarks`
  ADD CONSTRAINT `course_bookmarks_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_bookmarks_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_leader_sieves`
--
ALTER TABLE `course_leader_sieves`
  ADD CONSTRAINT `course_leader_sieves_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_leader_sieves_ibfk_2` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_moderators`
--
ALTER TABLE `course_moderators`
  ADD CONSTRAINT `course_moderators_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_moderators_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_participant_sieves`
--
ALTER TABLE `course_participant_sieves`
  ADD CONSTRAINT `course_participant_sieves_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_participant_sieves_ibfk_2` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_requirements`
--
ALTER TABLE `course_requirements`
  ADD CONSTRAINT `course_requirements_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_requirements_ibfk_2` FOREIGN KEY (`skill_id`) REFERENCES `skills` (`skill_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_supporter_sieves`
--
ALTER TABLE `course_supporter_sieves`
  ADD CONSTRAINT `course_supporter_sieves_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_supporter_sieves_ibfk_2` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE;

--
-- Constraints for table `events`
--
ALTER TABLE `events`
  ADD CONSTRAINT `events_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `events_ibfk_2` FOREIGN KEY (`location_id`) REFERENCES `locations` (`location_id`) ON UPDATE CASCADE;

--
-- Constraints for table `event_bookmarks`
--
ALTER TABLE `event_bookmarks`
  ADD CONSTRAINT `event_bookmarks_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_bookmarks_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_leader_filters`
--
ALTER TABLE `event_leader_filters`
  ADD CONSTRAINT `event_leader_filters_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_leader_filters_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_leader_presences`
--
ALTER TABLE `event_leader_presences`
  ADD CONSTRAINT `event_leader_presences_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_leader_presences_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_leader_registrations`
--
ALTER TABLE `event_leader_registrations`
  ADD CONSTRAINT `event_leader_registrations_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_leader_registrations_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_owners`
--
ALTER TABLE `event_owners`
  ADD CONSTRAINT `event_owners_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_owners_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_participant_filters`
--
ALTER TABLE `event_participant_filters`
  ADD CONSTRAINT `event_participant_filters_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_participant_filters_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_participant_presences`
--
ALTER TABLE `event_participant_presences`
  ADD CONSTRAINT `event_participant_presences_ibfk_2` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_participant_presences_ibfk_3` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_participant_registrations`
--
ALTER TABLE `event_participant_registrations`
  ADD CONSTRAINT `event_participant_registrations_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_participant_registrations_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_supporter_filters`
--
ALTER TABLE `event_supporter_filters`
  ADD CONSTRAINT `event_supporter_filters_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_supporter_filters_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_supporter_presences`
--
ALTER TABLE `event_supporter_presences`
  ADD CONSTRAINT `event_supporter_presences_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_supporter_presences_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `event_supporter_registrations`
--
ALTER TABLE `event_supporter_registrations`
  ADD CONSTRAINT `event_supporter_registrations_ibfk_1` FOREIGN KEY (`event_id`) REFERENCES `events` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `event_supporter_registrations_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Constraints for table `items`
--
ALTER TABLE `items`
  ADD CONSTRAINT `items_ibfk_1` FOREIGN KEY (`category_id`) REFERENCES `item_categories` (`category_id`) ON UPDATE CASCADE;

--
-- Constraints for table `team_members`
--
ALTER TABLE `team_members`
  ADD CONSTRAINT `team_members_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `team_members_ibfk_3` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE;

--
-- Constraints for table `terms`
--
ALTER TABLE `terms`
  ADD CONSTRAINT `terms_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `terms_ibfk_2` FOREIGN KEY (`club_id`) REFERENCES `clubs` (`club_id`) ON UPDATE CASCADE;

--
-- Constraints for table `user_competences`
--
ALTER TABLE `user_competences`
  ADD CONSTRAINT `user_competences_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `user_competences_ibfk_2` FOREIGN KEY (`skill_id`) REFERENCES `skills` (`skill_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `user_competences_ibfk_3` FOREIGN KEY (`judge_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

--
-- Constraints for table `user_possessions`
--
ALTER TABLE `user_possessions`
  ADD CONSTRAINT `user_possessions_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `user_possessions_ibfk_2` FOREIGN KEY (`item_id`) REFERENCES `items` (`item_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `user_possessions_ibfk_3` FOREIGN KEY (`club_id`) REFERENCES `clubs` (`club_id`) ON UPDATE CASCADE;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
