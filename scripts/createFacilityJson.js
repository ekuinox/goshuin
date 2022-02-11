const { readdirSync, writeFileSync } = require('fs');

// 宗教施設のjsonファイルの格納先
const FACILITIES_DIR = './facilities/';

// すべての施設のIDをリストで吐くファイル
const FACILITIES_FILE = './lib/facilities.json';

const dir = readdirSync(FACILITIES_DIR, { encoding: 'utf-8' });
const paths = dir.map((path) => {
    const m = path.match(/^(.+)\.json/);
    if (m == null) {
        return null;
    }
    return m[1];
}).filter((r) => r != null);

writeFileSync(FACILITIES_FILE, JSON.stringify(paths));
