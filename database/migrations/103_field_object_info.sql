CREATE TABLE "FieldObjectInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER,
    "PositionIndex" BIGINT -- References FieldObjectPositionInfo.InvenIndex
);