CREATE TABLE "EventHubInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "HubId" INTEGER,
    "StartTime" BIGINT,
    "PlayEndTime" BIGINT,
    "EndTime" BIGINT,
    "SettingInfoIndex" TEXT -- References EventHubSettingInfo.InvenIndex
);