const ff = require('ffjavascript');
const emailWalletUtils = require("../../utils");
import { hash_to_curve, point_scalar_mul } from "circom-grumpkin";
import { program } from "commander";
import fs from "fs";

program
    .requiredOption(
        "--x <string>",
        "X coordinate of the request point"
    )
    .requiredOption(
        "--y <string>",
        "Y coordinate of the request point"
    )
    .requiredOption(
        "--relayer-rand <string>",
        "Relayer's randomness"
    )
    .requiredOption(
        "--output <string>",
        "Path of a ouput file to write the randomness"
    )


program.parse();
const args = program.opts();

async function exec() {
    const responsePoint = point_scalar_mul(BigInt(args.x), BigInt(args.y), BigInt(args.relayerRand));
    const output = {
        x: responsePoint.x.toString(),
        y: responsePoint.y.toString()
    }
    fs.writeFileSync(args.output, JSON.stringify(output));
}


exec()
    .then(() => {
        process.exit(0);
    })
    .catch((err) => {
        console.log("Error: ", err);
        process.exit(1);
    });

