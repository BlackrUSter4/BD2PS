CREATE TABLE "MonsterHitInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "RageValue" INTEGER,
    "GroggyValue" INTEGER,
    "TargetPartsId" INTEGER,
    "TargetAttackType" INTEGER,
    "Damage" INTEGER,
    "AttackOwnerIndex" BIGINT,
    "ContactPoint" TEXT,
    "DisplayAttackCount" INTEGER,
    "IsCritical" INTEGER,
    "IsWeak" INTEGER
);