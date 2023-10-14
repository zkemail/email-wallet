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

contract AccountTransportVerifier {
    // Scalar field size
    uint256 constant r    = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q   = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax  = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay  = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1  = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2  = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1  = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2  = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 7763434763945176595176728700854437474986023373362325861046816220840806214880;
    uint256 constant deltax2 = 6150534978942951830651293849526872508625800507217710911404654137230283298677;
    uint256 constant deltay1 = 9538180923197496666883394556201218640900657880186594788846750592553259634430;
    uint256 constant deltay2 = 2804626981565407855707936868412206983670609371414906090654897205433756562784;

    
    uint256 constant IC0x = 7998753650107051487268468784645143542735123772722771857166048496175110214692;
    uint256 constant IC0y = 13662663758619547593519627299738670332099202715687378753046147257932978817230;
    
    uint256 constant IC1x = 1794503110363878482046663957627465514048599832919470986196341386102925837927;
    uint256 constant IC1y = 17044909380402348440239733324155340667398239954169674180199587194989291100621;
    
    uint256 constant IC2x = 8205165746718238388165971315964732105627118709945553436453034991754405833189;
    uint256 constant IC2y = 3763488108457599089206153299951674424428667163054797866652525772519751822753;
    
    uint256 constant IC3x = 7683578402928237061838375125025694213387577535328921836016418812577479610024;
    uint256 constant IC3y = 20281210533908310846682814061288559422485850652719333279350972457993497691666;
    
    uint256 constant IC4x = 8781614261679726049291444738363402411940389526825823591876359758048887787949;
    uint256 constant IC4y = 19249479422163359932813494972138189835540610842465724639897159188422521866340;
    
    uint256 constant IC5x = 7289467775423069966272705551051026242034145664048415239618415572156859741121;
    uint256 constant IC5y = 6589388755428239217296471607972545978158015793701053604811775071228882086318;
    
    uint256 constant IC6x = 2864998657577887477588948689833630555410144087352285526303558442388398578909;
    uint256 constant IC6y = 21472248766361960380723398274444126342415039023888069134257121192172012104292;
    
    uint256 constant IC7x = 5746898826177191239991820214004878094787387216160076764093310623925481058182;
    uint256 constant IC7y = 7278235692343048728581818522564103342321880608768201910320599738432488003112;
    
    uint256 constant IC8x = 3632301855909067898287056650395138203201531421385945007747773430770846286933;
    uint256 constant IC8y = 92002603893440441025414952492854206652487115256845547492211874286696425292;
    
    uint256 constant IC9x = 5857848404581424948259282070951410714924382004927359939141434312382867297662;
    uint256 constant IC9y = 21736887931949349580617129401327678393952949734867319084026266717589052128942;
    
    uint256 constant IC10x = 14964143462699248063368507454359194410759509387803571661989314277412936798816;
    uint256 constant IC10y = 15998240599321233186550170140449761261122702940223632300543597145439437790305;
    
    uint256 constant IC11x = 16231629822176814816908932385082629590746516146465995724740483059723401660409;
    uint256 constant IC11y = 19138477035378855869774274172592871234830090469081400975442625501968902466923;
    
    uint256 constant IC12x = 15221107068308301983567501128745011758514123116429802957685465039569353781635;
    uint256 constant IC12y = 19845049695028955810766990701837804163792988610303801710110590310958527115608;
    
    uint256 constant IC13x = 7073650364269319006499521706416090144145092740918695387958229584568987857395;
    uint256 constant IC13y = 13052945920099344551429155420511755485225165625701378063659909714516924730382;
    
    uint256 constant IC14x = 565369973321348674017292607680499349825583672391916582500461494917719425966;
    uint256 constant IC14y = 6723578735167733283092971593354841798272429696419251776302488106707596118502;
    
    uint256 constant IC15x = 15088651472715714186687843833365297389874087678131945912786988273393548969958;
    uint256 constant IC15y = 9848038045953199683366050370317691064486125203909999565362626917176240035995;
    
    uint256 constant IC16x = 10691818072240697162056257503870741285976401490264659719512808963446094751760;
    uint256 constant IC16y = 491948623825782692144802370208785748837396919004123294366560669850563855104;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[16] calldata _pubSignals) public view returns (bool) {
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
                
                g1_mulAccC(_pVk, IC16x, IC16y, calldataload(add(pubSignals, 480)))
                

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
            
            checkField(calldataload(add(_pubSignals, 512)))
            

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
             return(0, 0x20)
         }
     }
 }
