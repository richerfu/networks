import { constants } from "fs";
import { test, expect,describe } from "vitest";

import { resolveAddr } from "../dist/index";

describe('resolveAddr test', () => {
  test("should support inner dns server resolve", () => {
    const result = resolveAddr({
      addr: 'www.qq.com'
    });
    expect(result).toBeTruthy();
  });

  test("should support custom dns server resolve", () => {
    const result = resolveAddr({
      addr: 'www.qq.com'
    },{
      dnsServer: '114.114.114.114'
    });
    expect(result).toBeTruthy();
  });

  test("should return error with custom info",() => {
    try {
      resolveAddr({
        addr: 'a'
      })
    } catch (error) {
      console.log(error);
    expect(error).toThrowError(Error)
      
    }
  })
})
