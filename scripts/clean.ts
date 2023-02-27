import { clean } from "@qinglian/rimraf";
import * as glob from "glob";

const paths = [
  ...glob.sync("./packages/**/{.cargo,.turbo,dist,target,node_modules}"),
  ...glob.sync("./{.cargo,.turbo,dist,target,node_modules}"),
];

clean(paths);
