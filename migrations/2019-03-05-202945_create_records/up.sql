-- --------------------------------------------------------
-- Host:                         localhost
-- Server version:               10.3.13-MariaDB-1:10.3.13+maria~bionic-log - mariadb.org binary distribution
-- Server OS:                    debian-linux-gnu
-- HeidiSQL Version:             10.1.0.5464
-- --------------------------------------------------------

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET NAMES utf8 */;
/*!50503 SET NAMES utf8mb4 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;

-- Dumping structure for table obstacle_records.records
CREATE TABLE IF NOT EXISTS `records` (
  `time` int(11) NOT NULL,
  `respawn_count` int(11) NOT NULL,
  `try_count` int(11) NOT NULL DEFAULT 0,
  `created_at` datetime NOT NULL DEFAULT current_timestamp(),
  `updated_at` datetime NOT NULL DEFAULT current_timestamp(),
  `player_id` varchar(512) COLLATE utf8_unicode_ci NOT NULL,
  `map_id` varchar(512) COLLATE utf8_unicode_ci NOT NULL,
  PRIMARY KEY (`map_id`,`player_id`),
  FOREIGN KEY (`player_id`) REFERENCES `players`(`login`),
  FOREIGN KEY (`map_id`) REFERENCES `maps`(`maniaplanet_map_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

-- Dumping data for table obstacle_records.records: ~28 rows (approximately)
/*!40000 ALTER TABLE `records` DISABLE KEYS */;
INSERT INTO `records` (`time`, `respawn_count`, `try_count`, `created_at`, `updated_at`, `player_id`, `map_id`) VALUES
	(90440, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'guifox', '894NQmtJRcX37AhbL0wZmrrVPCb'),
	(201380, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'smokegun', '894NQmtJRcX37AhbL0wZmrrVPCb'),
	(22771, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'harkham', 'A5ap0hHnj4WINvMd9_sOjgaLvx1'),
	(27270, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'myke38', 'A5ap0hHnj4WINvMd9_sOjgaLvx1'),
	(64620, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'guifox', 'DtfW8eUqH8j8XvF12ZrpZ9nzxe7'),
	(77570, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'frilas', 'LHos6JroPkNAUzKay4YeFAZAsfe'),
	(69510, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'guifox', 'LHos6JroPkNAUzKay4YeFAZAsfe'),
	(84650, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'smokegun', 'LHos6JroPkNAUzKay4YeFAZAsfe'),
	(57110, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'frilas', 'pO2xeCSCzQp6szhL2tby4LDODYm'),
	(52010, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'guifox', 'pO2xeCSCzQp6szhL2tby4LDODYm'),
	(61350, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'harkham', 'pO2xeCSCzQp6szhL2tby4LDODYm'),
	(449370, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'myke38', 'pO2xeCSCzQp6szhL2tby4LDODYm'),
	(53640, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'smokegun', 'pO2xeCSCzQp6szhL2tby4LDODYm'),
	(103070, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'guifox', 'PSC9insg4cKTHl2zm_1ohg9nIX5'),
	(189590, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'bardak971', 'Q5ZSOHBcgBgtpAlcc86ggWYpEJ9'),
	(92320, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'smokegun', 'Q5ZSOHBcgBgtpAlcc86ggWYpEJ9'),
	(32460, -1, 17, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'harkham', 'RpMTjNcB_njgwS3BPsPz2EvT15k'),
	(36710, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'frilas', 'TZqxpYrSxmXbhkio4HCQg8PkcP5'),
	(38570, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'bardak971', 'uCehlz_85we5ZmD3k1fvPHHIXL5'),
	(38280, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'frilas', 'uCehlz_85we5ZmD3k1fvPHHIXL5'),
	(32150, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'guifox', 'uCehlz_85we5ZmD3k1fvPHHIXL5'),
	(33900, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'harkham', 'uCehlz_85we5ZmD3k1fvPHHIXL5'),
	(44240, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'myke38', 'uCehlz_85we5ZmD3k1fvPHHIXL5'),
	(33440, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'smokegun', 'uCehlz_85we5ZmD3k1fvPHHIXL5'),
	(82430, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'frilas', 'ZYrvKoq4T1SwSlq6r7v63huYUs0'),
	(45490, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'guifox', 'ZYrvKoq4T1SwSlq6r7v63huYUs0'),
	(46700, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'harkham', 'ZYrvKoq4T1SwSlq6r7v63huYUs0'),
	(47050, -1, 1, '2017-08-28 23:13:33', '2017-08-28 23:13:33', 'smokegun', 'ZYrvKoq4T1SwSlq6r7v63huYUs0');
/*!40000 ALTER TABLE `records` ENABLE KEYS */;

/*!40101 SET SQL_MODE=IFNULL(@OLD_SQL_MODE, '') */;
/*!40014 SET FOREIGN_KEY_CHECKS=IF(@OLD_FOREIGN_KEY_CHECKS IS NULL, 1, @OLD_FOREIGN_KEY_CHECKS) */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
