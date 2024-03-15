-- phpMyAdmin SQL Dump
-- version 5.2.1deb1ubuntu1
-- https://www.phpmyadmin.net/
--
-- Host: localhost:8001
-- Generation Time: Mar 15, 2024 at 08:05 AM
-- Server version: 10.11.6-MariaDB-0ubuntu0.23.10.2
-- PHP Version: 8.2.10-2ubuntu1

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
  `name` varchar(30) NOT NULL,
  `description` varchar(100) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `club_inventory`
--

CREATE TABLE `club_inventory` (
  `club_id` tinyint(4) NOT NULL,
  `item_id` int(11) NOT NULL,
  `count` int(11) NOT NULL
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
-- Table structure for table `course_moderators`
--

CREATE TABLE `course_moderators` (
  `course_id` mediumint(9) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `course_owner_teams`
--

CREATE TABLE `course_owner_teams` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `course_participant_teams`
--

CREATE TABLE `course_participant_teams` (
  `course_id` mediumint(9) NOT NULL,
  `team_id` mediumint(9) NOT NULL
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
-- Table structure for table `course_subscriptions`
--

CREATE TABLE `course_subscriptions` (
  `course_id` mediumint(9) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `items`
--

CREATE TABLE `items` (
  `item_id` int(11) NOT NULL,
  `item` varchar(30) NOT NULL,
  `category_id` smallint(6) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `item_category`
--

CREATE TABLE `item_category` (
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
  `title` varchar(100) NOT NULL
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
-- Table structure for table `slots`
--

CREATE TABLE `slots` (
  `slot_id` int(11) NOT NULL,
  `slot_key` char(12) NOT NULL,
  `pwd` tinytext NOT NULL,
  `title` varchar(100) NOT NULL,
  `location_id` smallint(6) NOT NULL,
  `begin` datetime NOT NULL,
  `end` datetime NOT NULL,
  `status` enum('DRAFT','PENDING','OCCURRING','CANCELED','REJECTED') NOT NULL DEFAULT 'PENDING',
  `public` tinyint(1) NOT NULL DEFAULT 0,
  `scrutable` tinyint(1) NOT NULL DEFAULT 1,
  `note` text NOT NULL DEFAULT '',
  `course_id` mediumint(9) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `slot_invites`
--

CREATE TABLE `slot_invites` (
  `slot_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `slot_owners`
--

CREATE TABLE `slot_owners` (
  `slot_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `slot_participants`
--

CREATE TABLE `slot_participants` (
  `slot_id` int(11) NOT NULL,
  `user_id` mediumint(9) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `teams`
--

CREATE TABLE `teams` (
  `team_id` mediumint(9) NOT NULL,
  `name` varchar(30) NOT NULL,
  `description` varchar(100) NOT NULL,
  `admin_competence` tinyint(1) NOT NULL DEFAULT 0,
  `admin_courses` tinyint(1) NOT NULL DEFAULT 0,
  `admin_event` tinyint(1) NOT NULL DEFAULT 0,
  `admin_inventory` tinyint(1) NOT NULL DEFAULT 0,
  `admin_teams` tinyint(1) NOT NULL DEFAULT 0,
  `admin_term` tinyint(1) NOT NULL DEFAULT 0,
  `admin_users` tinyint(1) NOT NULL DEFAULT 0
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
  `owner_id` tinyint(4) DEFAULT NULL,
  `transfer_date` date NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Indexes for dumped tables
--

--
-- Indexes for table `clubs`
--
ALTER TABLE `clubs`
  ADD PRIMARY KEY (`club_id`);

--
-- Indexes for table `club_inventory`
--
ALTER TABLE `club_inventory`
  ADD PRIMARY KEY (`club_id`,`item_id`),
  ADD KEY `REF_item` (`item_id`);

--
-- Indexes for table `courses`
--
ALTER TABLE `courses`
  ADD PRIMARY KEY (`course_id`),
  ADD UNIQUE KEY `KEY` (`course_key`);

--
-- Indexes for table `course_moderators`
--
ALTER TABLE `course_moderators`
  ADD PRIMARY KEY (`course_id`,`user_id`),
  ADD KEY `user_id` (`user_id`);

--
-- Indexes for table `course_owner_teams`
--
ALTER TABLE `course_owner_teams`
  ADD PRIMARY KEY (`course_id`,`team_id`),
  ADD KEY `team_id` (`team_id`);

--
-- Indexes for table `course_participant_teams`
--
ALTER TABLE `course_participant_teams`
  ADD PRIMARY KEY (`course_id`,`team_id`),
  ADD KEY `team_id` (`team_id`);

--
-- Indexes for table `course_requirements`
--
ALTER TABLE `course_requirements`
  ADD PRIMARY KEY (`requirement_id`),
  ADD KEY `REF_skill` (`skill_id`),
  ADD KEY `REF_course` (`course_id`);

--
-- Indexes for table `course_subscriptions`
--
ALTER TABLE `course_subscriptions`
  ADD PRIMARY KEY (`course_id`,`user_id`),
  ADD KEY `user_id` (`user_id`);

--
-- Indexes for table `items`
--
ALTER TABLE `items`
  ADD PRIMARY KEY (`item_id`),
  ADD KEY `REF_CATEGORY` (`category_id`);

--
-- Indexes for table `item_category`
--
ALTER TABLE `item_category`
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
-- Indexes for table `slots`
--
ALTER TABLE `slots`
  ADD PRIMARY KEY (`slot_id`),
  ADD UNIQUE KEY `KEY` (`slot_key`),
  ADD KEY `REF_course` (`course_id`),
  ADD KEY `REF_location` (`location_id`);

--
-- Indexes for table `slot_invites`
--
ALTER TABLE `slot_invites`
  ADD PRIMARY KEY (`slot_id`,`user_id`),
  ADD KEY `user_id` (`user_id`);

--
-- Indexes for table `slot_owners`
--
ALTER TABLE `slot_owners`
  ADD PRIMARY KEY (`slot_id`,`user_id`),
  ADD KEY `REF_user` (`user_id`),
  ADD KEY `REF_slot` (`slot_id`) USING BTREE;

--
-- Indexes for table `slot_participants`
--
ALTER TABLE `slot_participants`
  ADD PRIMARY KEY (`slot_id`,`user_id`),
  ADD KEY `user_id` (`user_id`);

--
-- Indexes for table `teams`
--
ALTER TABLE `teams`
  ADD PRIMARY KEY (`team_id`);

--
-- Indexes for table `team_members`
--
ALTER TABLE `team_members`
  ADD PRIMARY KEY (`user_id`,`team_id`),
  ADD KEY `team_id` (`team_id`);

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
  ADD KEY `REF_branch` (`skill_id`);

--
-- Indexes for table `user_possessions`
--
ALTER TABLE `user_possessions`
  ADD PRIMARY KEY (`possession_id`);

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
-- AUTO_INCREMENT for table `items`
--
ALTER TABLE `items`
  MODIFY `item_id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `item_category`
--
ALTER TABLE `item_category`
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
-- AUTO_INCREMENT for table `slots`
--
ALTER TABLE `slots`
  MODIFY `slot_id` int(11) NOT NULL AUTO_INCREMENT;

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
-- Constraints for table `club_inventory`
--
ALTER TABLE `club_inventory`
  ADD CONSTRAINT `club_inventory_ibfk_1` FOREIGN KEY (`club_id`) REFERENCES `clubs` (`club_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `club_inventory_ibfk_2` FOREIGN KEY (`item_id`) REFERENCES `items` (`item_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_moderators`
--
ALTER TABLE `course_moderators`
  ADD CONSTRAINT `course_moderators_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_moderators_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_owner_teams`
--
ALTER TABLE `course_owner_teams`
  ADD CONSTRAINT `course_owner_teams_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_owner_teams_ibfk_2` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_participant_teams`
--
ALTER TABLE `course_participant_teams`
  ADD CONSTRAINT `course_participant_teams_ibfk_2` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_participant_teams_ibfk_3` FOREIGN KEY (`team_id`) REFERENCES `teams` (`team_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_requirements`
--
ALTER TABLE `course_requirements`
  ADD CONSTRAINT `course_requirements_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_requirements_ibfk_2` FOREIGN KEY (`skill_id`) REFERENCES `skills` (`skill_id`) ON UPDATE CASCADE;

--
-- Constraints for table `course_subscriptions`
--
ALTER TABLE `course_subscriptions`
  ADD CONSTRAINT `course_subscriptions_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `course_subscriptions_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

--
-- Constraints for table `items`
--
ALTER TABLE `items`
  ADD CONSTRAINT `items_ibfk_1` FOREIGN KEY (`category_id`) REFERENCES `item_category` (`category_id`) ON UPDATE CASCADE;

--
-- Constraints for table `slots`
--
ALTER TABLE `slots`
  ADD CONSTRAINT `slots_ibfk_1` FOREIGN KEY (`course_id`) REFERENCES `courses` (`course_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `slots_ibfk_2` FOREIGN KEY (`location_id`) REFERENCES `locations` (`location_id`) ON UPDATE CASCADE;

--
-- Constraints for table `slot_invites`
--
ALTER TABLE `slot_invites`
  ADD CONSTRAINT `slot_invites_ibfk_1` FOREIGN KEY (`slot_id`) REFERENCES `slots` (`slot_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `slot_invites_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

--
-- Constraints for table `slot_owners`
--
ALTER TABLE `slot_owners`
  ADD CONSTRAINT `slot_owners_ibfk_1` FOREIGN KEY (`slot_id`) REFERENCES `slots` (`slot_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `slot_owners_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

--
-- Constraints for table `slot_participants`
--
ALTER TABLE `slot_participants`
  ADD CONSTRAINT `slot_participants_ibfk_2` FOREIGN KEY (`slot_id`) REFERENCES `slots` (`slot_id`) ON UPDATE CASCADE,
  ADD CONSTRAINT `slot_participants_ibfk_3` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON UPDATE CASCADE;

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
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
