CREATE TABLE "ServerInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Region" INTEGER,
    "GameServerInfo" TEXT,
    "CdnInfo" TEXT,
    "OpenFlag" INTEGER,
    "LogServerInfo" TEXT,
    "CharServerInfo" TEXT,
    "CouponWebInfo" TEXT,
    "GameDataInfo" TEXT,
    "GameDataVersion" TEXT
);