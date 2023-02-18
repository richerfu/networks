import { test, expect } from "vitest";

import { sum } from "../dist/index";

test("sum from native", () => {
  expect(sum(1, 2)).toBe(3);
});
