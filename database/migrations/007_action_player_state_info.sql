CREATE TABLE "ActionPlayerStateInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Type" INTEGER,
    "Seq" INTEGER,
    "OwnerIndex" BIGINT,
    "PlayerPosition" TEXT,
    "PlayerVector" TEXT,
    "KnockbackInfo" TEXT,
    "Speed" REAL,
    "DeltaTime" REAL,
    "SendTime" BIGINT,
    "Health" INTEGER,
    "Stamina" INTEGER,
    "RecoveryCount" INTEGER,
    "SkillId" INTEGER,
    "HitInfo" TEXT
);