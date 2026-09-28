CREATE TABLE "ActionUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "ActionCharId" INTEGER,
    "ClientState" INTEGER,
    "NetworkState" INTEGER,
    "NetworkPing" INTEGER,
    "CalcState" INTEGER
);