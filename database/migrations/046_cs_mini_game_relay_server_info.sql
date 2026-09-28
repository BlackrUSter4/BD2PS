CREATE TABLE "CSMiniGameRelayServerInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "HeaderIndex" BIGINT, -- References CommonHeader.InvenIndex
    "ServerInfoIndex" TEXT -- References RelayServerInfo.InvenIndex
);