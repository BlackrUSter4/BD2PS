CREATE TABLE "IdCardItemInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "Id" INTEGER,
    "X" INTEGER,
    "Y" INTEGER,
    "Rotate" REAL,
    "Scale" REAL,
    "Layer" INTEGER,
    "Color" TEXT
);