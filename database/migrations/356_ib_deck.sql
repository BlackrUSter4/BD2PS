CREATE TABLE "IbDeck" (
    "Uid" BIGINT NOT NULL,
    "Position" INTEGER NOT NULL,
    "InvenIndex" BIGINT NOT NULL,
    "RotationCount" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("Uid", "Position")
);
