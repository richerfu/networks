import * as rimraf from "rimraf";
import * as glob from "glob";

const clean = async () => {
  const paths = glob.sync(
    "./packages/**/{.cargo,.turbo,dist,target,node_modules}"
  );
  console.log("🫣 Start to clean some folders or files...");
  paths.forEach((i) => {
    rimraf.rimrafSync(i);
    console.log(`clean: ${i}`);
  });
  console.log("🌟 All tasks finished!");
};

clean();
