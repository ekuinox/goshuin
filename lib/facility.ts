import * as z from 'zod';

// 座標
export const degreeType = z.number();
export type Degree = z.infer<typeof degreeType>;

// 日付
export const dateType = z.string();

// 緯度経度
export const coordinateType = z.object({
    // 緯度
    lat: degreeType,

    // 経度
    lon: degreeType,
});
export type Coordinate = z.infer<typeof coordinateType>;

// 御朱印情報
export const goshuinType = z.object({
    // 最低1つ以上の画像
    pictureUrls: z.array(z.string()).min(1),

    // 説明文 (なくても良し)
    description: z.string().nullable().or(z.undefined()),

    // 日付文字列
    date: dateType,
});
export type Goshuin = z.infer<typeof goshuinType>;

// 宗教施設の種別
export const facilityKindType = z.union([
    // 寺
    z.literal('temple'),

    // 神社
    z.literal('shrine'),
]);
export type FacilityKind = z.infer<typeof facilityKindType>;

// 貰ったパンフだとかをくっつける
export const attachmentType = z.object({
    // 画像か何かのURL
    mediaUrl: z.string(),
    
    // 日付
    date: dateType,
});
export type Attachment = z.infer<typeof attachmentType>;

// 宗教施設
export const facilityType = z.object({
    // id なんでも良いので適当にユニークなのを自分でつける
    id: z.string(),

    // 施設の名前
    name: z.string(),

    // 種別
    kind: facilityKindType,

    // 座標
    coordinate: coordinateType,

    // 御朱印のリスト
    goshuinList: z.array(goshuinType),

    // メモ
    memo: z.string().nullable().or(z.undefined()),

    // 付属物
    attachments: z.array(attachmentType).nullable().or(z.undefined()),
});
export type Facility = z.infer<typeof facilityType>;

const idsType = z.array(z.string());

// idをすべて取得する
export const getFacilityIds = async (): Promise<Array<string>> => {
    const { default: ids } = await import('../lib/facilities.json');
    return idsType.parse(ids);
};

// 一件取得する
export const getFacility = async (id: string): Promise<Facility> => {
    const { default: facility } = await import(`../lib/facilities/${id}.json`);
    return facilityType.parse(facility);
};
