-- MySQL dump 10.13  Distrib 8.0.32, for Win64 (x86_64)
--
-- Host: 127.0.0.1    Database: levelcrush_accounts
-- ------------------------------------------------------
-- Server version	8.0.32

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Current Database: `levelcrush_accounts`
--

CREATE DATABASE /*!32312 IF NOT EXISTS*/ `levelcrush_accounts` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

USE `levelcrush_accounts`;

--
-- Table structure for table `account_platform_data`
--

DROP TABLE IF EXISTS `account_platform_data`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `account_platform_data` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `account` bigint NOT NULL,
  `platform` bigint NOT NULL,
  `key` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `value` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `value_bigint` bigint NOT NULL,
  `value_big` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `account_platform_data_account_platform_key` (`key`,`account`,`platform`),
  KEY `account_platform_data_account_index` (`account`),
  KEY `account_platform_data_key_index` (`key`),
  KEY `account_platform_data_platform_index` (`platform`),
  KEY `account_platform_data_value_index` (`value`),
  KEY `account_platform_data_value_bigint_index` (`value_bigint`)
) ENGINE=InnoDB AUTO_INCREMENT=131 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `account_platforms`
--

DROP TABLE IF EXISTS `account_platforms`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `account_platforms` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `account` bigint NOT NULL,
  `token` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `platform` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `platform_user` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `account_platforms_token` (`token`),
  KEY `account_platforms_account_index` (`account`),
  KEY `account_platforms_platform_index` (`platform`),
  KEY `account_platforms_platform_platform_user_index` (`platform`,`platform_user`),
  KEY `account_platforms_platform_user_index` (`platform_user`)
) ENGINE=InnoDB AUTO_INCREMENT=17 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `accounts`
--

DROP TABLE IF EXISTS `accounts`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `accounts` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `token` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `token_secret` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `timezone` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `admin` tinyint(1) NOT NULL,
  `last_login_at` bigint unsigned NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `accounts_token_secret_unique` (`token_secret`),
  UNIQUE KEY `accounts_token_unique` (`token`),
  KEY `accounts_timezone_index` (`timezone`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `application_users`
--

DROP TABLE IF EXISTS `application_users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `application_users` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `application` bigint NOT NULL,
  `account` bigint NOT NULL,
  `token` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `token_secret` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` int unsigned NOT NULL,
  `updated_at` int unsigned NOT NULL,
  `deleted_at` int unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `application_users_account_app_uk` (`account`,`application`),
  UNIQUE KEY `application_users_uk_app_token_token_secret` (`application`,`token`,`token_secret`),
  KEY `application_users_account_index` (`account`),
  KEY `application_users_application_index` (`application`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `applications`
--

DROP TABLE IF EXISTS `applications`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `applications` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `token` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `token_secret` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  PRIMARY KEY (`id`),
  UNIQUE KEY `applications_token` (`token`),
  UNIQUE KEY `applications_token_secret` (`token_secret`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `async_sessions`
--

DROP TABLE IF EXISTS `async_sessions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `async_sessions` (
  `id` varchar(128) NOT NULL,
  `expires` timestamp(6) NULL DEFAULT NULL,
  `session` text NOT NULL,
  PRIMARY KEY (`id`),
  KEY `expires` (`expires`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Current Database: `levelcrush_destiny`
--

CREATE DATABASE /*!32312 IF NOT EXISTS*/ `levelcrush_destiny` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

USE `levelcrush_destiny`;

--
-- Table structure for table `activities`
--

DROP TABLE IF EXISTS `activities`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `activities` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `activity_type` int unsigned NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `hash` int unsigned NOT NULL,
  `index` int unsigned NOT NULL,
  `is_pvp` tinyint(1) NOT NULL,
  `image_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `matchmaking_enabled` tinyint(1) NOT NULL,
  `fireteam_min_size` int unsigned NOT NULL,
  `fireteam_max_size` int unsigned NOT NULL,
  `max_players` int unsigned DEFAULT NULL,
  `requires_guardian_oath` tinyint(1) NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `activities_pk2` (`hash`),
  KEY `activities_activity_type_index` (`activity_type`),
  KEY `activities_index_index` (`index`),
  KEY `activities_name_index` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=2355 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `activity_types`
--

DROP TABLE IF EXISTS `activity_types`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `activity_types` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `hash` int unsigned NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `icon_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `index` int unsigned NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `activity_types_pk2` (`hash`)
) ENGINE=InnoDB AUTO_INCREMENT=68 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `clan_members`
--

DROP TABLE IF EXISTS `clan_members`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `clan_members` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `group_id` bigint NOT NULL,
  `group_role` tinyint NOT NULL,
  `membership_id` bigint NOT NULL,
  `platform` int NOT NULL,
  `joined_at` bigint unsigned NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `clan_members_group_id_index` (`group_id`),
  KEY `clan_members_membership_id_index` (`membership_id`),
  KEY `clan_members_group_role_index` (`group_role`),
  KEY `clan_members_group_id_membership_id_index` (`group_id`,`membership_id`),
  KEY `clan_members_group_role_index2` (`group_role`),
  KEY `clan_members_platform_index` (`platform`)
) ENGINE=InnoDB AUTO_INCREMENT=361 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `clans`
--

DROP TABLE IF EXISTS `clans`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `clans` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `group_id` bigint NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `slug` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `is_network` tinyint(1) NOT NULL,
  `motto` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `about` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `call_sign` varchar(12) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `clans_pk3` (`name`),
  UNIQUE KEY `clans_pk` (`group_id`),
  UNIQUE KEY `clans_pk2` (`slug`,`group_id`),
  KEY `clans_is_network_index` (`is_network`)
) ENGINE=InnoDB AUTO_INCREMENT=13 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `classes`
--

DROP TABLE IF EXISTS `classes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `classes` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `hash` int unsigned NOT NULL,
  `index` int unsigned NOT NULL,
  `type` tinyint unsigned NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `classes_pk2` (`hash`),
  KEY `classes_index_index` (`index`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `instance_members`
--

DROP TABLE IF EXISTS `instance_members`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `instance_members` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `instance_id` bigint NOT NULL,
  `membership_id` bigint NOT NULL,
  `platform` int NOT NULL,
  `character_id` bigint NOT NULL,
  `class_hash` int unsigned NOT NULL,
  `class_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `emblem_hash` int unsigned NOT NULL,
  `light_level` int NOT NULL,
  `clan_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `clan_tag` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `completed` tinyint(1) NOT NULL,
  `completion_reason` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `instance_members_pk2` (`instance_id`,`membership_id`),
  KEY `instance_members_instance_id_index` (`instance_id`),
  KEY `instance_members_membership_id_index` (`membership_id`),
  KEY `instance_members_clan_name_index` (`clan_name`),
  KEY `instance_members_clan_tag_index` (`clan_tag`),
  KEY `instance_members_class_hash_index` (`class_hash`),
  KEY `instance_members_class_name_index` (`class_name`),
  KEY `instance_members_emblem_hash_index` (`emblem_hash`),
  KEY `instance_members_character_id_index` (`character_id`),
  KEY `instance_members_completed_completion_reason_index` (`completed`,`completion_reason`),
  KEY `instance_members_completed_index` (`completed`),
  KEY `instance_members_completion_reason_index` (`completion_reason`),
  KEY `instance_members_platform_index` (`platform`)
) ENGINE=InnoDB AUTO_INCREMENT=3185341 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `instances`
--

DROP TABLE IF EXISTS `instances`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `instances` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `instance_id` bigint NOT NULL,
  `occurred_at` bigint unsigned NOT NULL,
  `starting_phase_index` int NOT NULL,
  `started_from_beginning` tinyint(1) NOT NULL,
  `activity_hash` int unsigned NOT NULL,
  `activity_director_hash` int unsigned NOT NULL,
  `is_private` tinyint(1) NOT NULL,
  `completed` tinyint(1) NOT NULL,
  `completion_reasons` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `instances_pk2` (`instance_id`),
  KEY `instances_activity_director_hash_index` (`activity_director_hash`),
  KEY `instances_activity_hash_index` (`activity_hash`),
  KEY `instances_is_private_index` (`is_private`),
  KEY `instances_started_from_beginning_index` (`started_from_beginning`),
  KEY `instances_completed_index` (`completed`),
  KEY `instances_completion_reasons_index` (`completion_reasons`)
) ENGINE=InnoDB AUTO_INCREMENT=553652 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `member_activities`
--

DROP TABLE IF EXISTS `member_activities`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `member_activities` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `membership_id` bigint NOT NULL,
  `character_id` bigint NOT NULL,
  `platform_played` int NOT NULL,
  `activity_hash` int unsigned NOT NULL,
  `activity_hash_director` int unsigned NOT NULL,
  `instance_id` bigint NOT NULL,
  `mode` int NOT NULL,
  `modes` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `private` tinyint(1) NOT NULL,
  `occurred_at` bigint unsigned NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `member_activities_activity_hash_director_index` (`activity_hash_director`),
  KEY `member_activities_activity_hash_index` (`activity_hash`),
  KEY `member_activities_character_id_index` (`character_id`),
  KEY `member_activities_membership_id_index` (`membership_id`),
  KEY `member_activities_mode_index` (`mode`),
  KEY `member_activities_platform_played_index` (`platform_played`),
  KEY `member_activities_private_index` (`private`),
  KEY `member_activities_instance_id_index` (`instance_id`),
  KEY `member_activities_instance_id_character_id_index` (`instance_id`,`character_id`),
  KEY `member_activities_membership_id_instance_id_index` (`membership_id`,`instance_id`)
) ENGINE=InnoDB AUTO_INCREMENT=638260 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `member_activity_stats`
--

DROP TABLE IF EXISTS `member_activity_stats`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `member_activity_stats` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `membership_id` bigint NOT NULL,
  `character_id` bigint NOT NULL,
  `instance_id` bigint NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `value` double NOT NULL,
  `value_display` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `member_activity_stats_pk2` (`membership_id`,`character_id`,`name`,`instance_id`),
  KEY `member_activity_stats_character_id_index` (`character_id`),
  KEY `member_activity_stats_instance_id_index` (`instance_id`),
  KEY `member_activity_stats_membership_id_index` (`membership_id`),
  KEY `member_activity_stats_name_index` (`name`),
  KEY `member_activity_stats_member_char_inst` (`membership_id`,`character_id`,`instance_id`),
  KEY `member_activity_stats_name_instance_id_character_id_index` (`name`,`instance_id`,`character_id`),
  KEY `member_activity_stats_name_instance_id_membership_id_index` (`name`,`instance_id`,`membership_id`),
  KEY `member_activity_stats_value_display_index` (`value_display`),
  KEY `member_activity_stats_value_index` (`value`)
) ENGINE=InnoDB AUTO_INCREMENT=12401643 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `member_character_stats`
--

DROP TABLE IF EXISTS `member_character_stats`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `member_character_stats` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `membership_id` bigint NOT NULL,
  `character_id` bigint NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `value` double NOT NULL,
  `value_display` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `member_character_stats_pk2` (`character_id`,`name`,`membership_id`),
  KEY `member_character_stats_character_id_index` (`character_id`),
  KEY `member_character_stats_membership_id_index` (`membership_id`),
  KEY `member_character_stats_name_index` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `member_characters`
--

DROP TABLE IF EXISTS `member_characters`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `member_characters` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `membership_id` bigint NOT NULL,
  `platform` int NOT NULL,
  `character_id` bigint NOT NULL,
  `class_hash` int unsigned NOT NULL,
  `light` int NOT NULL,
  `last_played_at` bigint unsigned NOT NULL,
  `minutes_played_session` int unsigned NOT NULL,
  `minutes_played_lifetime` int unsigned NOT NULL,
  `emblem_hash` int unsigned NOT NULL,
  `emblem_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `emblem_background_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `member_characters_id` (`character_id`),
  KEY `member_characters_class_hash_index` (`class_hash`),
  KEY `member_characters_membership_id_character_id_index` (`membership_id`,`character_id`),
  KEY `member_characters_membership_id_index` (`membership_id`),
  KEY `member_characters_platform_index` (`platform`),
  KEY `member_characters_emblem_hash_index` (`emblem_hash`)
) ENGINE=InnoDB AUTO_INCREMENT=793537 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `member_snapshots`
--

DROP TABLE IF EXISTS `member_snapshots`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `member_snapshots` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `membership_id` bigint NOT NULL,
  `snapshot_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `version` tinyint unsigned NOT NULL,
  `data` mediumtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `member_snapshots_pk` (`membership_id`,`snapshot_name`,`version`),
  KEY `member_snapshots_membership_id_index` (`membership_id`),
  KEY `member_snapshots_snapshot_name_index` (`snapshot_name`),
  KEY `member_snapshots_membership_id_snapshot_name_index` (`membership_id`,`snapshot_name`),
  KEY `member_snapshots_version_index` (`version`)
) ENGINE=InnoDB AUTO_INCREMENT=145 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `member_stats`
--

DROP TABLE IF EXISTS `member_stats`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `member_stats` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `membership_id` bigint NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `value` double NOT NULL,
  `value_display` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `member_stats_pk2` (`name`,`membership_id`),
  KEY `member_stats_membership_id_index` (`membership_id`),
  KEY `member_stats_name_index` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `member_triumphs`
--

DROP TABLE IF EXISTS `member_triumphs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `member_triumphs` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `membership_id` bigint NOT NULL,
  `hash` int unsigned NOT NULL,
  `state` int NOT NULL,
  `times_completed` int NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `member_triumphs_pk2` (`membership_id`,`hash`),
  KEY `member_triumphs_hash_index` (`hash`),
  KEY `member_triumphs_membership_id_index` (`membership_id`),
  KEY `member_triumphs_state_index` (`state`),
  KEY `member_triumphs_times_completed_index` (`times_completed`)
) ENGINE=InnoDB AUTO_INCREMENT=21643609 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `members`
--

DROP TABLE IF EXISTS `members`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `members` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `membership_id` bigint NOT NULL,
  `platform` int NOT NULL,
  `display_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `display_name_global` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `guardian_rank_current` tinyint unsigned NOT NULL,
  `guardian_rank_lifetime` tinyint unsigned NOT NULL,
  `last_played_at` bigint unsigned NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `members_membership_id` (`membership_id`),
  KEY `members_display_name_global_index` (`display_name_global`),
  KEY `members_display_name_index` (`display_name`),
  KEY `members_platform_index` (`platform`),
  KEY `members_last_played_at_index` (`last_played_at`)
) ENGINE=InnoDB AUTO_INCREMENT=275895 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `seasons`
--

DROP TABLE IF EXISTS `seasons`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `seasons` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `hash` int unsigned NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `pass_hash` int unsigned NOT NULL,
  `number` int NOT NULL,
  `starts_at` bigint unsigned NOT NULL,
  `ends_at` bigint unsigned NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `seasons_pk3` (`number`),
  UNIQUE KEY `seasons_pk2` (`hash`),
  KEY `seasons_name_index` (`name`),
  KEY `seasons_pass_hash_index` (`pass_hash`)
) ENGINE=InnoDB AUTO_INCREMENT=24 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `setting_modes`
--

DROP TABLE IF EXISTS `setting_modes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `setting_modes` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `leaderboard` tinyint NOT NULL,
  `dashboard` tinyint NOT NULL,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `value` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `order` int NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `setting_modes_unique_name` (`name`),
  KEY `setting_modes_dashboard_index` (`dashboard`),
  KEY `setting_modes_leaderboard_index` (`leaderboard`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `triumphs`
--

DROP TABLE IF EXISTS `triumphs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `triumphs` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `hash` int unsigned NOT NULL,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `is_title` tinyint NOT NULL,
  `gilded` tinyint NOT NULL,
  `created_at` bigint unsigned NOT NULL,
  `updated_at` bigint unsigned NOT NULL,
  `deleted_at` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `triumphs_pk2` (`hash`),
  KEY `triumphs_gilded_index` (`gilded`),
  KEY `triumphs_is_title_index` (`is_title`),
  KEY `triumphs_name_index` (`name`),
  KEY `triumphs_title_index` (`title`)
) ENGINE=InnoDB AUTO_INCREMENT=3920 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Current Database: `levelcrush_feeds`
--

CREATE DATABASE /*!32312 IF NOT EXISTS*/ `levelcrush_feeds` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

USE `levelcrush_feeds`;

--
-- Table structure for table `access_keys`
--

DROP TABLE IF EXISTS `access_keys`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `access_keys` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `application` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `public_key` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `private_key` char(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` int unsigned NOT NULL,
  `updated_at` int unsigned NOT NULL,
  `deleted_at` int unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `access_keys_application` (`application`),
  UNIQUE KEY `access_keys_private_key` (`private_key`),
  UNIQUE KEY `access_keys_public_key` (`public_key`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `feeds`
--

DROP TABLE IF EXISTS `feeds`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `feeds` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `access_key` bigint NOT NULL,
  `slug` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `data` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` int unsigned NOT NULL,
  `updated_at` int unsigned NOT NULL,
  `deleted_at` int unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `feeds_access_slug` (`access_key`,`slug`),
  KEY `feeds_access_key_index` (`access_key`),
  KEY `feeds_slug_index` (`slug`)
) ENGINE=InnoDB AUTO_INCREMENT=29 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Current Database: `levelcrush_settings`
--

CREATE DATABASE /*!32312 IF NOT EXISTS*/ `levelcrush_settings` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

USE `levelcrush_settings`;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-06-21  4:29:30
