CREATE TABLE "LifeWorldObjectInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ChunkId" INTEGER NOT NULL,
    "ObjectIndex" INTEGER,
    "ObjectId" INTEGER,
    "X" INTEGER,
    "Y" INTEGER,
    "Rotate" INTEGER,
    "Status" INTEGER,
    "StartTime" BIGINT,
    "EndTime" BIGINT,
    "ParentIndex" BIGINT
);
