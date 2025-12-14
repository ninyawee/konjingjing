import { describe, expect, test } from "bun:test";
import { getIdMeaning } from "../index.js";

describe("getIdMeaning", () => {
  test("valid ID returns meaning", () => {
    const result = getIdMeaning("1112034563562");
    expect(result).not.toBeNull();
    expect(result!.personTypeCode).toBe(1);
    expect(result!.personTypeDescription).toBe("คนไทยที่แจ้งเกิดภายในกำหนด");
    expect(result!.personTypeDescriptionEn).toBe(
      "Thai citizen, birth registered on time"
    );
    expect(result!.provinceCode).toBe(11);
    expect(result!.provinceNameTh).toBe("สมุทรปราการ");
    expect(result!.provinceNameEn).toBe("Samut Prakan");
    expect(result!.isValid).toBe(true);
  });

  test("Bangkok ID", () => {
    const result = getIdMeaning("1101700230703");
    expect(result).not.toBeNull();
    expect(result!.personTypeCode).toBe(1);
    expect(result!.provinceCode).toBe(10);
    expect(result!.provinceNameTh).toBe("กรุงเทพมหานคร");
    expect(result!.provinceNameEn).toBe("Bangkok");
    expect(result!.amphoeCode).toBe(1017);
    expect(result!.amphoeName).toBe("ห้วยขวาง");
  });

  test("person type 2 (late registration)", () => {
    const result = getIdMeaning("2101700230702");
    expect(result).not.toBeNull();
    expect(result!.personTypeCode).toBe(2);
    expect(result!.personTypeDescription).toContain("แจ้งเกิดเกินกำหนด");
    expect(result!.personTypeDescriptionEn).toBe(
      "Thai citizen, birth registered late"
    );
  });

  test("invalid format returns null", () => {
    expect(getIdMeaning("")).toBeNull();
    expect(getIdMeaning("123")).toBeNull();
    expect(getIdMeaning("abcdefghijklm")).toBeNull();
  });

  test("invalid person type returns null", () => {
    expect(getIdMeaning("0101700230704")).toBeNull();
    expect(getIdMeaning("9101700230701")).toBeNull();
  });

  test("unknown province returns undefined province fields", () => {
    const result = getIdMeaning("1991700230700");
    expect(result).not.toBeNull();
    expect(result!.provinceCode).toBeUndefined();
    expect(result!.provinceNameTh).toBeUndefined();
  });
});
