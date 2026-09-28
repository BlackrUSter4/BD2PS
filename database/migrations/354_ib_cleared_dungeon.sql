CREATE TABLE "IbClearedDungeon" (
    "Uid" BIGINT NOT NULL,
    "Season" INTEGER NOT NULL,
    "DungeonId" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "Season", "DungeonId")
);
