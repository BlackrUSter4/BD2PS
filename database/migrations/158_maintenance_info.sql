CREATE TABLE "MaintenanceInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "MarketType" INTEGER,
    "Version" TEXT,
    "BundleVersion" TEXT,
    "IsBundleUpdate" INTEGER,
    "MaintenanceType" INTEGER,
    "Date" TEXT,
    "RegionList" TEXT,
    "UseDsa" INTEGER,
    "MaintenanceUrl" TEXT,
    "UseMaintenanceUrl" INTEGER,
    "DownloadUrl" TEXT,
    "Notice" TEXT,
    "BundleVersionSd" TEXT
);