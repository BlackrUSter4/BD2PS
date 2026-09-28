-- Fixes latent schema/model mismatches in the Equip tables: these tables and their Rust
-- models/queries were scaffolded together but never actually exercised (every equip gameserver
-- handler was an empty TODO stub until this round), so several mismatches between the migration
-- SQL and the FromRow model went unnoticed until now.
ALTER TABLE "EquipInfo" RENAME COLUMN "BaseInfo" TO "BaseInfoIndex";
ALTER TABLE "EquipBaseInfo" RENAME COLUMN "PrivateOption" TO "PrivateOptionIndex";
ALTER TABLE "EquipBaseInfo" ADD COLUMN "MainOptionIndex" TEXT;
ALTER TABLE "EquipBaseInfo" ADD COLUMN "SubOptionIndex" TEXT;
ALTER TABLE "EquipBaseInfo" ADD COLUMN "Rank" INTEGER NOT NULL DEFAULT 1;
ALTER TABLE "EquipBatchUseInfo" ADD COLUMN "EquipInvenIndex" BIGINT NOT NULL DEFAULT 0;
ALTER TABLE "EquipClearInfo" ADD COLUMN "EquipInvenIndex" BIGINT NOT NULL DEFAULT 0;
ALTER TABLE "EquipPresetInfo" ADD COLUMN "ItemInfoIndex" TEXT;
ALTER TABLE "EquipInfo" ADD COLUMN "Mark" TEXT;
-- Stashes the pre-reroll option state so EquipOptionReRollConfirm(is_confirm=false) can restore
-- it; EquipOptionReRoll writes the new options immediately (simplifying the round-trip) and
-- populates these, EquipOptionReRollConfirm clears them either way.
ALTER TABLE "EquipBaseInfo" ADD COLUMN "PrevMainOptionIndex" TEXT;
ALTER TABLE "EquipBaseInfo" ADD COLUMN "PrevSubOptionIndex" TEXT;
