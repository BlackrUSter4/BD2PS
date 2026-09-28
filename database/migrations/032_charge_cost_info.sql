CREATE TABLE "ChargeCostInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CostTimeInfoIndex" INTEGER NOT NULL, -- References CostTimeInfo.InvenIndex
    "EventScheduleInfoIndex" INTEGER -- References ChargeCostEventScheduleInfo.InvenIndex
);
