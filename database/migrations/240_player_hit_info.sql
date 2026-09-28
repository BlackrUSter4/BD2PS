CREATE TABLE "PlayerHitInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "AttackCharId" INTEGER,
    "AttackSkillId" INTEGER,
    "ContactPoint" TEXT
);