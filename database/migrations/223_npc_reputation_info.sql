CREATE TABLE "NpcReputationInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PackId" INTEGER,
    "GroupId" INTEGER,
    "NpcId" INTEGER,
    "Point" INTEGER
);