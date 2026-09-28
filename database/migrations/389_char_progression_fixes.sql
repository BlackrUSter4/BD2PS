-- Adds storage this round's Char stub-fixes need that the pre-existing
-- scaffolded schema didn't have a column/table for yet.

-- CharClassUp needs to persist a class/tier stage per character (no
-- equivalent column existed at all).
ALTER TABLE "CharInfo" ADD COLUMN "ClassLevel" INTEGER NOT NULL DEFAULT 0;

-- CharAutoReviveSet needs a small per-account settings row (no table existed).
CREATE TABLE "CharAutoReviveSetting" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL UNIQUE,
    "CanAutoRevive" INTEGER NOT NULL DEFAULT 0,
    "CastingCharInvenIndex" BIGINT
);
