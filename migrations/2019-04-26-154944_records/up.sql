/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `records` (
  `rank` int unsigned NOT NULL,
  `time` int(11) NOT NULL,
  `respawn_count` int(11) NOT NULL,
  `try_count` int(11) NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  `player_id` varchar(512) COLLATE utf8_unicode_ci NOT NULL,
  `map_id` varchar(512) COLLATE utf8_unicode_ci NOT NULL,
  FOREIGN KEY (map_id) REFERENCES maps(maniaplanet_map_id),
  FOREIGN KEY (player_id) REFERENCES players(login),
  PRIMARY KEY (`map_id`,`player_id`),
  KEY `updated_at` (`updated_at`),
  KEY `time` (`time`),
  KEY `rank` (`rank`),
  KEY `map_id` (`map_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
