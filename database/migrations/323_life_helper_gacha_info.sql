CREATE TABLE "LifeHelperGachaInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "HelperSlotId" INTEGER,
    "HelperId" INTEGER,
    "HelperName" TEXT,
    "UseCharId" INTEGER,
    "UseHairId" INTEGER,
    "UseHairAccessoryId" INTEGER,
    "UseFaceAccessoryId" INTEGER,
    "UseCostumeId" INTEGER,
    "UseBodyAccessoryId" INTEGER,
    "UseHandAccessoryId" INTEGER,
    "UsePetId" INTEGER,
    "UseMountId" INTEGER,
    "UseEffectId" INTEGER,
    "AvatarDate" BIGINT
);
