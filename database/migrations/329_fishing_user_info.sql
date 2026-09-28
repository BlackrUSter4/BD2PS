CREATE TABLE "FishingUserInfo" (
    "Uid" BIGINT PRIMARY KEY,
    "Exp" INTEGER NOT NULL DEFAULT 0,
    "Level" INTEGER NOT NULL DEFAULT 1,
    "BoatLevel" INTEGER NOT NULL DEFAULT 1,
    "BoatSkinId" INTEGER,
    "UseRodInvenIndex" BIGINT,
    "MultiApResetTime" BIGINT,
    "TrapRewardReceiptTime" BIGINT,
    "FishInvenSlotCount" INTEGER NOT NULL DEFAULT 20
);
