import { test, expect,describe } from "vitest";

import { resolveAddr } from "../dist/index";

describe('resolveAddr test', () => {
  test("should support inner dns server resolve", () => {
    const result = resolveAddr({
      addr: 'www.qq.com'
    });
    console.log(result);
    expect(result).toBeTruthy();
  });

  test("should support custom dns server resolve", () => {
    const result = resolveAddr({
      addr: 'www.qq.com'
    },{
      dnsServer: '114.114.114.114'
    });
    console.log(result);
    expect(result).toBeTruthy();
  });
})
