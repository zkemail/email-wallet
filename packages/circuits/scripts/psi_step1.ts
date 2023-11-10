const ff = require('ffjavascript');
const emailWalletUtils = require("../../utils");
import { hash_to_curve, point_scalar_mul } from "circom-grumpkin";
import { program } from "commander";
import fs from "fs";

program
    .requiredOption(
        "--email-addr <string>",
        "User's email address"
    )
    .requiredOption(
        "--client-rand <string>",
        "Temporary random scalar"
    )
    .requiredOption(
        "--output <string>",
        "Path of a ouput file to write the randomness"
    )


program.parse();
const args = program.opts();

async function exec() {
    const emailAddr = args.emailAddr;
    const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
    const hashedPoint = hash_to_curve(paddedEmailAddr);
    const requestPoint = point_scalar_mul(hashedPoint.x, hashedPoint.y, BigInt(args.clientRand));
    const output = {
        x: requestPoint.x.toString(),
        y: requestPoint.y.toString()
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


