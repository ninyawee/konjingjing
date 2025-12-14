import { describe, expect, test } from "bun:test";
import { verifyId, verify_id } from "../index.js";

describe("verifyId", () => {
  test("valid ID", () => {
    expect(verifyId("1112034563562")).toBe(true);
  });

  test("invalid checksum", () => {
    expect(verifyId("1101700230705")).toBe(false);
  });

  test("too short", () => {
    expect(verifyId("110170023073")).toBe(false);
  });

  test("contains letters", () => {
    expect(verifyId("11017002070d3")).toBe(false);
    expect(verifyId("rytege54fsfsf")).toBe(false);
  });

  test("single char", () => {
    expect(verifyId("0")).toBe(false);
    expect(verifyId("-")).toBe(false);
  });

  test("empty string", () => {
    expect(verifyId("")).toBe(false);
  });

  test("garbage input", () => {
    expect(verifyId("blablabla")).toBe(false);
  });
});
