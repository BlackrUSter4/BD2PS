CREATE TABLE "CharInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "Id" INTEGER,
    "Hp" BIGINT,
    "Level" INTEGER,
    "CostumeId" INTEGER,
    "Exp" INTEGER,
    "UseCostume" BIGINT,
    "TalentLevel" INTEGER,
    "TalentExp" INTEGER,
    "SolidarityReward" INTEGER,
    "ExpiryTime" BIGINT,
    "PictorialbookInfoIndex" TEXT, -- References PictorialBookInfo.InvenIndex
    "ConnectPotentialCostume" INTEGER
);