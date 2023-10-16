// SPDX-License-Identifier: GPL-3.0
/*
    Copyright 2021 0KIMS association.

    This file is generated with [snarkJS](https://github.com/iden3/snarkjs).

    snarkJS is a free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    snarkJS is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
    License for more details.

    You should have received a copy of the GNU General Public License
    along with snarkJS. If not, see <https://www.gnu.org/licenses/>.
*/

pragma solidity >=0.7.0 <0.9.0;

contract AccountInitVerifier {
    // Scalar field size
    uint256 constant r = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1 = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2 = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1 = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2 = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 2030021325825475856670431033207444952105651272057739788687761811272595311446;
    uint256 constant deltax2 = 19837053358224654616692768278456496920439578845065197766449450590168152406791;
    uint256 constant deltay1 = 2520928781130170062112208786915617000782037580811373438540595910554078217715;
    uint256 constant deltay2 = 3323310874808353596400661768263049071880070203679522930395821821676551260605;

    uint256 constant IC0x = 21738926515288706243967543128016150099722851910570281027784694044544647277076;
    uint256 constant IC0y = 8348567993736401727805535001650325371129789445721342897986097894677709545879;

    uint256 constant IC1x = 19847243317148364693345794073900561384074139285495578779673889389529366775994;
    uint256 constant IC1y = 5892088711395901925957300881754959482123513028303608393593930769558792685685;

    uint256 constant IC2x = 7148549509760129816156703047971888479989933950720670960105153058701320887793;
    uint256 constant IC2y = 16199064867938692253151782062252501136580532361941388975853649926880504956084;

    uint256 constant IC3x = 12204415784606152536050014763485023368525574351638469681156809498545381860567;
    uint256 constant IC3y = 1166402614904177418613385912840627911438470976972803352364309298939410685021;

    uint256 constant IC4x = 1780679183917750259974088440429101231855554530866720767019259720809868398441;
    uint256 constant IC4y = 19364708761464267194957996585733172137273408665582272802994611921982802546271;

    uint256 constant IC5x = 5458832857098072802428664673093147657281029912219191708817180744071355639495;
    uint256 constant IC5y = 12178065314619911681276322535239882338317848332402809721665887083707258686002;

    uint256 constant IC6x = 13957252801952981046248120366234718557780571197905056312927060146552736273129;
    uint256 constant IC6y = 20427982311953918981258477954764871187697146996523665801549399882478905496691;

    uint256 constant IC7x = 2552739075919624982021188721818748298940963008437489373325396943810509322944;
    uint256 constant IC7y = 9671416280587512343119072776909002678310872946717906027956809682230311809328;

    uint256 constant IC8x = 18430229854438690699771560016758824088444400277838495284724574161662947187855;
    uint256 constant IC8y = 1229980949943176344200525007790777341983305690612290798313567093799192268747;

    uint256 constant IC9x = 12059263202726592736725307593092709516464374822850523071603831825227250917781;
    uint256 constant IC9y = 13786639599949563686158740847651221251976541783262526342808957061259696667762;

    uint256 constant IC10x = 2798973883079907706160398491319718669606458239788592856453925547274222670438;
    uint256 constant IC10y = 17771947387279124661279851043563083067833106689663950314162676722589508976154;

    uint256 constant IC11x = 12032842099629716605407579603805419260614257471864872605462340694653007964393;
    uint256 constant IC11y = 18684969892991020662938023316902935328890959702301831414807267310634698057378;

    uint256 constant IC12x = 5103135276059324657838572424981533717057813578612062804756159120345817050913;
    uint256 constant IC12y = 16138399643266339010119286147823867303305400960155901546905760621176277643006;

    uint256 constant IC13x = 13570791852929366386525352253004108907955098981885771869636533675450429272272;
    uint256 constant IC13y = 6099148014169817455764666560782135770310262809492543687998509993812643994416;

    uint256 constant IC14x = 1349812184752029100828981085705476690543776108480340105387190127546255112058;
    uint256 constant IC14y = 7328370573150485341945476847227774973320359399569928986249279500345424210176;

    uint256 constant IC15x = 10703671464261495761316088455152005123723127397443101242016890332570244935351;
    uint256 constant IC15y = 21281690951573098365237106257714558780791985541142725494635170158797692199312;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint[2] calldata _pA,
        uint[2][2] calldata _pB,
        uint[2] calldata _pC,
        uint[15] calldata _pubSignals
    ) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, q)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(pA, pB, pC, pubSignals, pMem) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, IC0x)
                mstore(add(_pVk, 32), IC0y)

                // Compute the linear combination vk_x

                g1_mulAccC(_pVk, IC1x, IC1y, calldataload(add(pubSignals, 0)))

                g1_mulAccC(_pVk, IC2x, IC2y, calldataload(add(pubSignals, 32)))

                g1_mulAccC(_pVk, IC3x, IC3y, calldataload(add(pubSignals, 64)))

                g1_mulAccC(_pVk, IC4x, IC4y, calldataload(add(pubSignals, 96)))

                g1_mulAccC(_pVk, IC5x, IC5y, calldataload(add(pubSignals, 128)))

                g1_mulAccC(_pVk, IC6x, IC6y, calldataload(add(pubSignals, 160)))

                g1_mulAccC(_pVk, IC7x, IC7y, calldataload(add(pubSignals, 192)))

                g1_mulAccC(_pVk, IC8x, IC8y, calldataload(add(pubSignals, 224)))

                g1_mulAccC(_pVk, IC9x, IC9y, calldataload(add(pubSignals, 256)))

                g1_mulAccC(_pVk, IC10x, IC10y, calldataload(add(pubSignals, 288)))

                g1_mulAccC(_pVk, IC11x, IC11y, calldataload(add(pubSignals, 320)))

                g1_mulAccC(_pVk, IC12x, IC12y, calldataload(add(pubSignals, 352)))

                g1_mulAccC(_pVk, IC13x, IC13y, calldataload(add(pubSignals, 384)))

                g1_mulAccC(_pVk, IC14x, IC14y, calldataload(add(pubSignals, 416)))

                g1_mulAccC(_pVk, IC15x, IC15y, calldataload(add(pubSignals, 448)))

                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(add(_pPairing, 32), mod(sub(q, calldataload(add(pA, 32))), q))

                // B
                mstore(add(_pPairing, 64), calldataload(pB))
                mstore(add(_pPairing, 96), calldataload(add(pB, 32)))
                mstore(add(_pPairing, 128), calldataload(add(pB, 64)))
                mstore(add(_pPairing, 160), calldataload(add(pB, 96)))

                // alpha1
                mstore(add(_pPairing, 192), alphax)
                mstore(add(_pPairing, 224), alphay)

                // beta2
                mstore(add(_pPairing, 256), betax1)
                mstore(add(_pPairing, 288), betax2)
                mstore(add(_pPairing, 320), betay1)
                mstore(add(_pPairing, 352), betay2)

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))

                // gamma2
                mstore(add(_pPairing, 448), gammax1)
                mstore(add(_pPairing, 480), gammax2)
                mstore(add(_pPairing, 512), gammay1)
                mstore(add(_pPairing, 544), gammay2)

                // C
                mstore(add(_pPairing, 576), calldataload(pC))
                mstore(add(_pPairing, 608), calldataload(add(pC, 32)))

                // delta2
                mstore(add(_pPairing, 640), deltax1)
                mstore(add(_pPairing, 672), deltax2)
                mstore(add(_pPairing, 704), deltay1)
                mstore(add(_pPairing, 736), deltay2)

                let success := staticcall(sub(gas(), 2000), 8, _pPairing, 768, _pPairing, 0x20)

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F

            checkField(calldataload(add(_pubSignals, 0)))

            checkField(calldataload(add(_pubSignals, 32)))

            checkField(calldataload(add(_pubSignals, 64)))

            checkField(calldataload(add(_pubSignals, 96)))

            checkField(calldataload(add(_pubSignals, 128)))

            checkField(calldataload(add(_pubSignals, 160)))

            checkField(calldataload(add(_pubSignals, 192)))

            checkField(calldataload(add(_pubSignals, 224)))

            checkField(calldataload(add(_pubSignals, 256)))

            checkField(calldataload(add(_pubSignals, 288)))

            checkField(calldataload(add(_pubSignals, 320)))

            checkField(calldataload(add(_pubSignals, 352)))

            checkField(calldataload(add(_pubSignals, 384)))

            checkField(calldataload(add(_pubSignals, 416)))

            checkField(calldataload(add(_pubSignals, 448)))

            checkField(calldataload(add(_pubSignals, 480)))

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
