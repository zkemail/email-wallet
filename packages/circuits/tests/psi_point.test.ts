const ff = require('ffjavascript');
const emailWalletUtils = require("../../utils");
import { hash_to_curve, point_scalar_mul } from "circom-grumpkin";

// const grumpkin = require("circom-grumpkin");
jest.setTimeout(120000);
describe("PSI point", () => {
    it("psi point calculation test", async () => {
        const emailAddr = "suegamisora@gmail.com";
        const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
        const hashedPoint = hash_to_curve(paddedEmailAddr);
        const relayerRand = emailWalletUtils.genRelayerRand();
        const clientRand = emailWalletUtils.genRelayerRand();
        const requestPoint = point_scalar_mul(hashedPoint.x, hashedPoint.y, BigInt(clientRand));
        const responsePoint = point_scalar_mul(requestPoint.x, requestPoint.y, BigInt(relayerRand));
        const scalarField = new ff.F1Field("21888242871839275222246405745257275088696311157297823662689037894645226208583");
        const invClientRand = scalarField.inv(BigInt(clientRand));
        const invedPoint = point_scalar_mul(responsePoint.x, responsePoint.y, invClientRand);
        const psiPoint = point_scalar_mul(hashedPoint.x, hashedPoint.y, BigInt(relayerRand));
        expect(psiPoint.x).toEqual(invedPoint.x);
        expect(psiPoint.y).toEqual(invedPoint.y);
    });
});

