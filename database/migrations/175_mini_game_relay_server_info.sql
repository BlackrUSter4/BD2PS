CREATE TABLE "MiniGameRelayServerInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ServerInfoIndex" TEXT -- References RelayServerInfo.InvenIndex
);