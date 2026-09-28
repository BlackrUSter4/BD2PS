-- The pre-existing EvilCastle* tables (075-098) were scaffolded but never
-- exercised by real gameserver logic. Fixing a few latent gaps found while
-- wiring real logic to them, same as the Equip/MyRoom rounds' schema fixes:
--   * EvilCastleInfo had no way to distinguish multiple towers (pack_id) per account.
--   * EvilCastleRogueLikeChoiceInfo was missing the actual "repeated id" list.
--   * EvilCastleRogueLikeRoomInfo had no floor association, so rooms from
--     different floors of the same run couldn't be told apart.
ALTER TABLE "EvilCastleInfo" ADD COLUMN "PackId" INTEGER NOT NULL DEFAULT 0;
ALTER TABLE "EvilCastleRogueLikeChoiceInfo" ADD COLUMN "Ids" TEXT NOT NULL DEFAULT '';
ALTER TABLE "EvilCastleRogueLikeRoomInfo" ADD COLUMN "Floor" INTEGER NOT NULL DEFAULT 0;

-- New: the roguelike mode needs its own deck (separate from the account's
-- main team deck, same "one deck table per feature" convention already used
-- by Colosseum/Guild/PvP/MonsterHunt/Field/Preset/Recommend/Supporter/TotalWar).
CREATE TABLE "EvilCastleRogueLikeDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInvenIndex" BIGINT NOT NULL,
    "Position" INTEGER NOT NULL,
    "Sequence" INTEGER NOT NULL DEFAULT 0
);

-- New: daily reward claim tracking (no pre-existing table covered this).
CREATE TABLE "EvilCastleDailyRewardInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL UNIQUE,
    "LastClaimDate" TEXT,
    "ClaimCount" INTEGER NOT NULL DEFAULT 0
);
