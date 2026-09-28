CREATE TABLE "DefenseUnitInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UnitId" INTEGER,
    "UnitIndex" INTEGER,
    "SpawnTicks" BIGINT,
    "DespwanTicks" BIGINT,
    "ElementLevel" INTEGER,
    "CoolTime" REAL,
    "GridIndex" INTEGER
);