import { test, expect } from "vitest";

import { createPingInstance } from "../dist/index";

test("sum from native", () => {
  expect(createPingInstance()).toStrictEqual({
    addr: '12',
    count: 1,
    timeout: 3000
  });
});
