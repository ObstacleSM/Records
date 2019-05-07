/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `maps` (
  `maniaplanet_map_id` varchar(255) COLLATE utf8_unicode_ci NOT NULL,
  `name` varchar(512) COLLATE utf8_unicode_ci NOT NULL,
  `player_id` varchar(512) COLLATE utf8_unicode_ci NOT NULL,
  FOREIGN KEY (player_id) REFERENCES players(login),
  PRIMARY KEY (`maniaplanet_map_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
