-- MySQL dump 10.13  Distrib 8.4.4, for Win64 (x86_64)
--
-- Host: localhost    Database: demo
-- ------------------------------------------------------
-- Server version	8.4.4

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
-- Table structure for table `sys_dept`
--

DROP TABLE IF EXISTS `sys_dept`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_dept` (
  `id` char(32) NOT NULL COMMENT '部门id',
  `name` varchar(30) DEFAULT NULL COMMENT '部门名称',
  `email` varchar(50) DEFAULT NULL COMMENT '邮箱',
  `telephone` varchar(11) DEFAULT NULL COMMENT '联系电话',
  `address` varchar(200) DEFAULT NULL COMMENT '地址',
  `logo` varchar(100) DEFAULT NULL COMMENT 'logo地址',
  `parent_id` char(32) DEFAULT NULL COMMENT '父部门id',
  `seq_no` int DEFAULT NULL COMMENT '显示顺序',
  `status` int DEFAULT NULL COMMENT '部门状态(0正常 1停用)',
  `create_by` varchar(30) DEFAULT NULL COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(30) DEFAULT NULL COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(200) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='部门表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_dept`
--

LOCK TABLES `sys_dept` WRITE;
/*!40000 ALTER TABLE `sys_dept` DISABLE KEYS */;
INSERT INTO `sys_dept` VALUES 
('065a3eb180214ccfbb653f63287d285d','NB PLUS 科技',NULL,NULL,NULL,NULL,'','1',0,'admin','2024-09-25 17:29:32',NULL,NULL,NULL),
('0753bee6314c4f6fa01cb5166e6e4b14','市场部','1234567@163.com','12345678901','dfasdfasdfasdfsfasldfasdfasdfasdfasdfasdfasdfasdfsadf','https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg','6bcbfa6a71fa454d8363b885ccbcb30a',1,0,'admin','2024-10-01 01:40:07','admin','2024-10-07 22:09:42','sad发送到发送到发送到发送到发送到发送到发送地方'),
('3fd2fbc8c93e4a20a6e46387b36c0264','销售部',NULL,NULL,NULL,NULL,'6bcbfa6a71fa454d8363b885ccbcb30a',1,0,'admin','2024-09-25 17:38:06',NULL,NULL,NULL),
('6bcbfa6a71fa454d8363b885ccbcb30a','XXX总公司',NULL,NULL,NULL,NULL,'065a3eb180214ccfbb653f63287d285d',1,0,'admin','2024-09-25 17:35:22',NULL,NULL,NULL),
('88921a7b70a04ea2b7d1db1651ffe666','研发部',NULL,NULL,NULL,'https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg','6bcbfa6a71fa454d8363b885ccbcb30a',1,1,'admin','2024-09-26 01:37:41','admin','2024-10-05 13:00:15',NULL),
('a308936d3b5d49e185fe15bf7d71b688','XXX分公司',NULL,NULL,NULL,NULL,'065a3eb180214ccfbb653f63287d285d',1,1,'admin','2024-09-25 17:43:24',NULL,NULL,NULL),
('ebd8118fe94247809abab8c72579cf7d','运维',NULL,NULL,NULL,NULL,'a308936d3b5d49e185fe15bf7d71b688',NULL,0,'admin','2024-10-13 19:34:06',NULL,NULL,NULL),
('f9271a783e074715aceeb4d9b427004d','财务部',NULL,NULL,NULL,NULL,'6bcbfa6a71fa454d8363b885ccbcb30a',1,0,'admin','2024-09-26 01:39:45','admin','2024-10-31 17:38:07',NULL);
/*!40000 ALTER TABLE `sys_dept` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_menu`
--

DROP TABLE IF EXISTS `sys_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_menu` (
  `id` char(32) NOT NULL COMMENT '菜单ID',
  `name` varchar(30) DEFAULT NULL COMMENT '菜单名称',
  `parent_id` char(32) DEFAULT NULL COMMENT '父菜单ID',
  `seq_no` int DEFAULT NULL COMMENT '显示顺序',
  `menu_type` char(1) DEFAULT NULL COMMENT '菜单类型（D目录 M菜单 B按钮',
  `url` varchar(200) DEFAULT NULL COMMENT '请求地址',
  `perms` varchar(100) DEFAULT NULL COMMENT '权限标识',
  `status` int DEFAULT NULL COMMENT '菜单状态(0停用 1正常)',
  `hidden` int DEFAULT NULL COMMENT '是否在侧边栏隐藏(0显示 1隐藏)',
  `always_show` int DEFAULT NULL COMMENT '是否始终显示根菜单(0隐藏 1显示)',
  `redirect` varchar(200) DEFAULT NULL COMMENT '重定向地址，当设置 noRedirect 的时候该路由在面包屑导航中不可被点击',
  `component` varchar(200) DEFAULT NULL COMMENT '当前路由外层包裹的组件信息(嵌套路由)',
  `href` varchar(200) DEFAULT NULL COMMENT '外部链接地址',
  `icon` varchar(200) DEFAULT NULL COMMENT '侧边栏中显示的图标',
  `no_cache` int DEFAULT NULL COMMENT '不缓存页面(0缓存 1不缓存)',
  `affix` int DEFAULT NULL COMMENT '页面附加在标签视图中(0不附加 1附加)',
  `breadcrumb` int DEFAULT NULL COMMENT '该项目将隐藏在breadcrumb中(0隐藏 1显示)',
  `active_menu` varchar(200) DEFAULT NULL COMMENT ' 如果设置路径，侧边栏会突出显示您设置的路径(例: /example/list)',
  `create_by` varchar(30) DEFAULT NULL COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(30) DEFAULT NULL COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(200) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='菜单权限表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_menu`
--

LOCK TABLES `sys_menu` WRITE;
/*!40000 ALTER TABLE `sys_menu` DISABLE KEYS */;
INSERT INTO `sys_menu` VALUES ('0471759cae664f9e910df35b22610397','重置密码','0fdf3ff4c5b348deb960e989e6ee83af',6,'B',NULL,'system:user:resetPwd',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 21:46:17','admin','2025-04-13 11:51:31',NULL),('0ed02fde4a13409d8e0d7d50e1afdee9','角色详情','abb861bd5da94a4e884b5e70758754be',2,'B',NULL,'system:role:detail',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('0fdf3ff4c5b348deb960e989e6ee83af','async_router.user_management','d815a5167ad84c2aab29aa9ab080ca7c',2,'M','user','system:user:list',1,0,0,NULL,'views/system/user',NULL,'ep:user',0,0,1,NULL,'admin','2024-03-09 13:15:19',NULL,NULL,NULL),('1d5879088287492098a1adf2cf4b3bd3','菜单详情','c1ab532b91fb405fb223ef679a656c8f',2,'B',NULL,'system:menu:detail',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('233daa1502c344d685adc9341d11d1e9','async_router.dept_management','d815a5167ad84c2aab29aa9ab080ca7c',1,'M','dept','system:dept:list',1,0,0,NULL,'views/system/dept',NULL,'clarity:tree-view-solid',0,0,1,NULL,'admin','2024-03-09 13:19:38','admin','2025-01-01 13:13:22','vgsd '),('3779ea2c1666420a8e1e4f7bd5556305','编辑部门','233daa1502c344d685adc9341d11d1e9',3,'B',NULL,'system:dept:edit',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('4ee4ba300f9e4baa8a319b13a8eba9a8','新增部门','233daa1502c344d685adc9341d11d1e9',1,'B',NULL,'system:dept:add',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('6c099c5d5f044d50b22ef89337e2a2fc','部门详情','233daa1502c344d685adc9341d11d1e9',2,'B',NULL,'system:dept:detail',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('7aaf9913860a4510b709026ab7a424a2','新增菜单','c1ab532b91fb405fb223ef679a656c8f',1,'B',NULL,'system:menu:add',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('7d0a685469044d549409059459b0a30c','编辑菜单','c1ab532b91fb405fb223ef679a656c8f',3,'B',NULL,'system:menu:edit',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('94da045292c94840ab35f97920784c3b','删除用户','0fdf3ff4c5b348deb960e989e6ee83af',4,'B',NULL,'system:user:remove',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('9ac1f8a41eae449fa5a0656f90ae88f0','async_router.system_doc','d815a5167ad84c2aab29aa9ab080ca7c',0,'M','doc','system:doc:list',0,0,0,NULL,'/views/iframe','http://localhost:15775/doc.html','ep:document',1,0,0,NULL,'admin','2024-03-09 13:19:38','admin','2024-12-31 12:42:25',NULL),('a36d7bcfc2a441359a48b00331a33e37','删除角色','abb861bd5da94a4e884b5e70758754be',4,'B',NULL,'system:role:remove',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('a519fd3031a6445d91a8ac06d7a169ea','新增用户','0fdf3ff4c5b348deb960e989e6ee83af',1,'B',NULL,'system:user:add',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('abb861bd5da94a4e884b5e70758754be','async_router.role_management','d815a5167ad84c2aab29aa9ab080ca7c',3,'M','role','system:role:list',1,0,0,NULL,'views/system/role',NULL,'eos-icons:role-binding-outlined',0,0,1,NULL,'admin','2024-03-09 13:18:03',NULL,NULL,NULL),('acd5dd88d3dd4154bbfb56899dfc1eb6','删除部门','233daa1502c344d685adc9341d11d1e9',4,'B',NULL,'system:dept:remove',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('b75d2b9178f84a34bb5b5bad67efc030','新增角色','abb861bd5da94a4e884b5e70758754be',1,'B',NULL,'system:role:add',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('b787bac121734a13bc37caed22e402a0','分配角色','0fdf3ff4c5b348deb960e989e6ee83af',5,'B',NULL,'system:user:setUserRole',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 21:46:17','admin','2025-04-13 11:51:02',NULL),('bc9ae5bd610848abae58f0f56387c7df','删除菜单','c1ab532b91fb405fb223ef679a656c8f',4,'B',NULL,'system:menu:remove',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('c083a729a60447d18893fe3f4d206d6c','编辑用户','0fdf3ff4c5b348deb960e989e6ee83af',3,'B',NULL,'system:user:edit',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('c1ab532b91fb405fb223ef679a656c8f','async_router.menu_management','d815a5167ad84c2aab29aa9ab080ca7c',4,'M','menu','system:menu:list',1,0,0,NULL,'views/system/menu',NULL,'ep:menu',0,0,1,NULL,'admin','2024-03-09 13:19:09',NULL,NULL,NULL),('d040bcff21954c748f862645d2b43711','用户详情','0fdf3ff4c5b348deb960e989e6ee83af',2,'B',NULL,'system:user:detail',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL),('d815a5167ad84c2aab29aa9ab080ca7c','async_router.system_management',NULL,0,'M','/system','system',1,0,0,'/system/user','layout',NULL,'ep:setting',0,0,1,'','admin','2024-03-07 20:15:00','admin','2025-01-01 13:16:54',NULL),('e93ac37073b84f569f46b307e9ab4cc6','分配菜单','abb861bd5da94a4e884b5e70758754be',1,'B',NULL,'system:role:setRoleMenu',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 21:46:17',NULL,NULL,NULL),('ec0b6e0593ea42c3931a594b49c50f30','编辑角色','abb861bd5da94a4e884b5e70758754be',3,'B',NULL,'system:role:edit',1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,'admin','2025-04-08 20:50:19',NULL,NULL,NULL);
/*!40000 ALTER TABLE `sys_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role`
--

DROP TABLE IF EXISTS `sys_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role` (
  `id` char(32) NOT NULL COMMENT '角色ID',
  `name` varchar(30) DEFAULT NULL COMMENT '角色名称',
  `role_key` varchar(100) DEFAULT NULL COMMENT '角色权限字符串',
  `seq_no` int DEFAULT NULL COMMENT '显示顺序',
  `status` int DEFAULT NULL COMMENT '角色状态(0停用 1正常)',
  `create_by` varchar(30) DEFAULT NULL COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(30) DEFAULT NULL COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(200) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_role_key` (`role_key`),
  UNIQUE KEY `uk_rolename` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role`
--

LOCK TABLES `sys_role` WRITE;
/*!40000 ALTER TABLE `sys_role` DISABLE KEYS */;
INSERT INTO `sys_role` VALUES ('36f0df335b6a4293b3ebd8e7a27e6026','超级管理员','admin',0,1,'admin','2024-03-07 20:13:55','admin','2025-04-13 15:55:00',NULL),('3f536e29949b4723a8fb4514411d50df','一般人员','common',0,1,'common','2024-03-07 20:13:33','admin','2025-04-13 15:55:03',NULL);
/*!40000 ALTER TABLE `sys_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_role_menu`
--

DROP TABLE IF EXISTS `sys_role_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_role_menu` (
  `role_id` char(32) NOT NULL COMMENT '角色ID',
  `menu_id` char(32) NOT NULL COMMENT '菜单ID',
  PRIMARY KEY (`role_id`,`menu_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色和菜单关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_role_menu`
--

LOCK TABLES `sys_role_menu` WRITE;
/*!40000 ALTER TABLE `sys_role_menu` DISABLE KEYS */;
INSERT INTO `sys_role_menu` VALUES ('36f0df335b6a4293b3ebd8e7a27e6026','0471759cae664f9e910df35b22610397'),('36f0df335b6a4293b3ebd8e7a27e6026','0ed02fde4a13409d8e0d7d50e1afdee9'),('36f0df335b6a4293b3ebd8e7a27e6026','0fdf3ff4c5b348deb960e989e6ee83af'),('36f0df335b6a4293b3ebd8e7a27e6026','1d5879088287492098a1adf2cf4b3bd3'),('36f0df335b6a4293b3ebd8e7a27e6026','233daa1502c344d685adc9341d11d1e9'),('36f0df335b6a4293b3ebd8e7a27e6026','3779ea2c1666420a8e1e4f7bd5556305'),('36f0df335b6a4293b3ebd8e7a27e6026','4ee4ba300f9e4baa8a319b13a8eba9a8'),('36f0df335b6a4293b3ebd8e7a27e6026','6c099c5d5f044d50b22ef89337e2a2fc'),('36f0df335b6a4293b3ebd8e7a27e6026','7aaf9913860a4510b709026ab7a424a2'),('36f0df335b6a4293b3ebd8e7a27e6026','7d0a685469044d549409059459b0a30c'),('36f0df335b6a4293b3ebd8e7a27e6026','94da045292c94840ab35f97920784c3b'),('36f0df335b6a4293b3ebd8e7a27e6026','a36d7bcfc2a441359a48b00331a33e37'),('36f0df335b6a4293b3ebd8e7a27e6026','a519fd3031a6445d91a8ac06d7a169ea'),('36f0df335b6a4293b3ebd8e7a27e6026','abb861bd5da94a4e884b5e70758754be'),('36f0df335b6a4293b3ebd8e7a27e6026','acd5dd88d3dd4154bbfb56899dfc1eb6'),('36f0df335b6a4293b3ebd8e7a27e6026','b75d2b9178f84a34bb5b5bad67efc030'),('36f0df335b6a4293b3ebd8e7a27e6026','b787bac121734a13bc37caed22e402a0'),('36f0df335b6a4293b3ebd8e7a27e6026','bc9ae5bd610848abae58f0f56387c7df'),('36f0df335b6a4293b3ebd8e7a27e6026','c083a729a60447d18893fe3f4d206d6c'),('36f0df335b6a4293b3ebd8e7a27e6026','c1ab532b91fb405fb223ef679a656c8f'),('36f0df335b6a4293b3ebd8e7a27e6026','d040bcff21954c748f862645d2b43711'),('36f0df335b6a4293b3ebd8e7a27e6026','d815a5167ad84c2aab29aa9ab080ca7c'),('36f0df335b6a4293b3ebd8e7a27e6026','e93ac37073b84f569f46b307e9ab4cc6'),('36f0df335b6a4293b3ebd8e7a27e6026','ec0b6e0593ea42c3931a594b49c50f30');
/*!40000 ALTER TABLE `sys_role_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user`
--

DROP TABLE IF EXISTS `sys_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user` (
  `id` char(32) NOT NULL COMMENT '用户ID',
  `dept_id` char(32) DEFAULT NULL COMMENT '部门ID',
  `name` varchar(30) DEFAULT NULL COMMENT '用户账号',
  `email` varchar(50) DEFAULT NULL COMMENT '用户邮箱',
  `phone_number` varchar(11) DEFAULT NULL COMMENT '手机号码',
  `sex` char(1) DEFAULT NULL COMMENT '用户性别(0未知 1男 2女)',
  `password` varchar(100) DEFAULT NULL COMMENT '密码',
  `avatar` varchar(100) DEFAULT NULL COMMENT '头像',
  `status` int DEFAULT NULL COMMENT '账号状态(0停用 1正常)',
  `login_ip` varchar(128) DEFAULT NULL COMMENT '最后登录IP',
  `login_time` datetime DEFAULT NULL COMMENT '最后登录时间',
  `create_by` varchar(30) DEFAULT NULL COMMENT '创建者',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_by` varchar(30) DEFAULT NULL COMMENT '更新者',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  `remark` varchar(200) DEFAULT NULL COMMENT '备注',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_username` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户信息表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user`
--

LOCK TABLES `sys_user` WRITE;
/*!40000 ALTER TABLE `sys_user` DISABLE KEYS */;
INSERT INTO `sys_user` VALUES ('0aa01bb4d3f7423f9e11336cf5ca6ef5','88921a7b70a04ea2b7d1db1651ffe666','admin','1593437442@qq.com','17625291384','1','$2a$10$yosWu//m6rWvGDkGjzxcW.pyPbTuGgqj35kZpzgIb9mktx/K59zU.',NULL,1,NULL,NULL,'admin','2024-08-24 21:43:28',NULL,NULL,NULL),('9057fae81b7345e2ac725346278fb9bb','88921a7b70a04ea2b7d1db1651ffe666','测试用户','12345678@qq.com','12345678901','1','$2a$10$55V2B27H30SSDoFEz10uneAINEIt3LsVGGTPvuLynAmmDQY6Njo76',NULL,1,NULL,NULL,'admin','2024-03-09 13:27:33','admin','2025-04-15 13:14:43',NULL);
/*!40000 ALTER TABLE `sys_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `sys_user_role`
--

DROP TABLE IF EXISTS `sys_user_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `sys_user_role` (
  `user_id` char(32) NOT NULL COMMENT '用户ID',
  `role_id` char(32) NOT NULL COMMENT '角色ID',
  PRIMARY KEY (`user_id`,`role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户和角色关联表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `sys_user_role`
--

LOCK TABLES `sys_user_role` WRITE;
/*!40000 ALTER TABLE `sys_user_role` DISABLE KEYS */;
INSERT INTO `sys_user_role` VALUES ('03a3aee395b04c43b2bc08af426fe0a8','36f0df335b6a4293b3ebd8e7a27e6026'),('0aa01bb4d3f7423f9e11336cf5ca6ef5','36f0df335b6a4293b3ebd8e7a27e6026'),('9057fae81b7345e2ac725346278fb9bb','3f536e29949b4723a8fb4514411d50df');
/*!40000 ALTER TABLE `sys_user_role` ENABLE KEYS */;
UNLOCK TABLES;

/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-04-17 10:17:58