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
    const scalarField = new ff.F1Field("21888242871839275222246405745257275088696311157297823662689037894645226208583");
    const invClientRand = scalarField.inv(BigInt(args.clientRand));
    const outputPoint = point_scalar_mul(BigInt(args.x), BigInt(args.y), invClientRand);
    const output = {
        x: outputPoint.x.toString(),
        y: outputPoint.y.toString()
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

