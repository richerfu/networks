import { test, expect } from "vitest";

import { Ping } from "../dist/index";

test("Ping success", () => {
  const ping = new Ping({
    addr: 'www.baidu.com'
  })
  const result = ping.ping();
  console.log(result);
  expect(result).toBeTruthy();
});
