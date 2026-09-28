CREATE TABLE "MyRoomItemPositionInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "ObjectType" INTEGER,
    "PositionType" INTEGER,
    "X" INTEGER,
    "Y" INTEGER,
    "Rotate" INTEGER,
    "Interact" INTEGER,
    "ItemAnimation" INTEGER,
    "IsWallHidden" INTEGER
);