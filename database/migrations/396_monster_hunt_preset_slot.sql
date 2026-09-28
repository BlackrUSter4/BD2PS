-- MonsterHuntPresetInfo (202_monster_hunt_preset_info.sql) had no way to address an
-- individual preset slot even though MonsterHuntPresetDeleteRequest/InfoChangeRequest/
-- UseRequest all address presets by slot number. Adding the missing column.
ALTER TABLE "MonsterHuntPresetInfo" ADD COLUMN "Slot" INTEGER;
ALTER TABLE "MonsterHuntPresetInfo" ADD COLUMN "PresetName" TEXT;
ALTER TABLE "MonsterHuntPresetInfo" ADD COLUMN "PresetResourceId" INTEGER;
ALTER TABLE "MonsterHuntPresetInfo" ADD COLUMN "PresetResourceColor" INTEGER;
