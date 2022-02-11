import z from 'zod';

// 座標
export const degreeType = z.number();
export type Degree = z.infer<typeof degreeType>;

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
    description: z.string().nullable(),

    // 日付文字列
    date: z.string(), // このまま `new Date()` に食わせる
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
    date: z.string(),
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
    memo: z.string().nullable(),

    // 付属物
    attachments: z.array(attachmentType).nullable(),
});
export type Facility = z.infer<typeof facilityType>;
