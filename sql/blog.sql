SET NAMES utf8mb4;
SET
FOREIGN_KEY_CHECKS = 0;

DROP TABLE IF EXISTS `blog`;
CREATE TABLE `blog`
(
    `id`          int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `title`       varchar(255) DEFAULT NULL COMMENT '标题',
    `description` varchar(255) DEFAULT NULL COMMENT '描述',
    PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb4;

SET
FOREIGN_KEY_CHECKS = 1;

