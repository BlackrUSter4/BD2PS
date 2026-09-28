CREATE TABLE "LifeCitizenInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CitizenIndex" INTEGER,
    "CitizenSlotId" INTEGER,
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
