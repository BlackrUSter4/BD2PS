CREATE TABLE IF NOT EXISTS HuntingGroundMonster (
    "HuntingGroundIndex" INTEGER NOT NULL,
    "MonsterIndex"       INTEGER NOT NULL,
    FOREIGN KEY("HuntingGroundIndex") REFERENCES HuntingGroundInfo("Index") ON DELETE CASCADE,
    FOREIGN KEY("MonsterIndex")       REFERENCES MonsterInfo("Index")       ON DELETE CASCADE
);
