CREATE TABLE "ReputationInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER,
    "State" INTEGER,
    "ElapsedSeconds" INTEGER
);